
use ndarray::*;
use opcua::server::prelude::*;
use local_ip_address::local_ip;
use opcua::server::config;
use teh_o_prke::{control_rod_feedback::obtain_rod_worth_cylinder, fuel_temperature_feedback::SimpleFuelTemperatureFeedback, zero_power_prke::six_group::SixGroupPRKE};
use uom::si::{energy::megaelectronvolt, f64::*, length::centimeter, linear_number_density::per_meter, power::kilowatt, ratio::ratio, thermodynamic_temperature::{degree_celsius, kelvin}, time::{microsecond, nanosecond}, velocity::meter_per_second, volumetric_number_density::per_cubic_meter, volumetric_number_rate::per_cubic_meter_second};

use std::{ops::{Deref, DerefMut}, sync::{Arc, Mutex}, time::{Instant, SystemTime}};
fn main(){
    let run_server = true;
    construct_and_run_fuel_temp_control_rod_prke_server_delayed_critical(run_server);
    //construct_and_run_fuel_temp_control_rod_prke_server_prompt_critical(run_server);
}


#[warn(missing_docs)]


/// runs an opcua server with initial delayed criticality
pub fn construct_and_run_fuel_temp_control_rod_prke_server_delayed_critical(run_server: bool){

    let mut server = build_standard_server();

    let ns = {
        let address_space = server.address_space();
        let mut address_space = address_space.write();
        address_space
            .register_namespace("urn:simple-server")
            .unwrap()
    };

    // i'll have some variables here.
    // most important is to give user control of reactivity
    let control_rod_set_point_node_cm = NodeId::new(ns, "control_rod_set_point_input_cm");


    // the resulting outputs are precursor concentrations and neutron population

    let reactor_power_node = NodeId::new(ns, "reactor_power_kilowatts");
    let neutron_concentration_node = NodeId::new(ns, "neutron_concentration_per_m3");
    let precursor_concentration_1_node = NodeId::new(ns, "precursor_concentration_1_per_m3");
    let precursor_concentration_2_node = NodeId::new(ns, "precursor_concentration_2_per_m3");
    let precursor_concentration_3_node = NodeId::new(ns, "precursor_concentration_3_per_m3");
    let precursor_concentration_4_node = NodeId::new(ns, "precursor_concentration_4_per_m3");
    let precursor_concentration_5_node = NodeId::new(ns, "precursor_concentration_5_per_m3");
    let precursor_concentration_6_node = NodeId::new(ns, "precursor_concentration_6_per_m3");
    let fuel_temperature_node = NodeId::new(ns, "fuel_temperature_output_celsius");

    let address_space = server.address_space();

    // this is the piece of code for the writeonly variable
    // we can use booleans or floats
    {
        let mut address_space = address_space.write();
        let folder_id = address_space
            .add_folder("Controller", "Controller", &NodeId::objects_folder_id())
            .unwrap();


        // we start with negative reactivity first
        VariableBuilder::new(&control_rod_set_point_node_cm, 
                             "control_rod_set_point_input_cm", "control_rod_set_point_input_cm")
            .data_type(DataTypeId::Float)
            .value(45.0 as f64)
            .writable()
            .organized_by(&folder_id)
            .insert(&mut address_space);

    }
    // this part is responsible for sensor data
    {
        let mut address_space = address_space.write();

        // Create a sample folder under objects folder
        let sample_folder_id = address_space
            .add_folder("sensor data", "sensor data", &NodeId::objects_folder_id())
            .unwrap();

        // Add some variables to our sample folder. Values will be overwritten by the timer
        let _ = address_space.add_variables(
            vec![

                Variable::new(&reactor_power_node, 
                              "reactor_power_kilowatts", 
                              "reactor_power_kilowatts", 0 as f64),
                Variable::new(&neutron_concentration_node, 
                              "neutron_concentration_per_m3", 
                              "neutron_concentration_per_m3", 0 as f64),
                Variable::new(&precursor_concentration_1_node, 
                              "precursor_concentration_1_per_m3", 
                              "precursor_concentration_1_per_m3", 0 as f64),
                Variable::new(&precursor_concentration_2_node, 
                              "precursor_concentration_2_per_m3", 
                              "precursor_concentration_2_per_m3", 0 as f64),
                Variable::new(&precursor_concentration_3_node, 
                              "precursor_concentration_3_per_m3", 
                              "precursor_concentration_3_per_m3", 0 as f64),
                Variable::new(&precursor_concentration_4_node, 
                              "precursor_concentration_4_per_m3", 
                              "precursor_concentration_4_per_m3", 0 as f64),
                Variable::new(&precursor_concentration_5_node, 
                              "precursor_concentration_5_per_m3", 
                              "precursor_concentration_5_per_m3", 0 as f64),
                Variable::new(&precursor_concentration_6_node, 
                              "precursor_concentration_6_per_m3", 
                              "precursor_concentration_6_per_m3", 0 as f64),
                Variable::new(&fuel_temperature_node, 
                              "fuel_temperature_output_celsius", 
                              "fuel_temperature_output_celsius", 0 as f64),
            ],
            &sample_folder_id,
        );
    }

    //
    // this one prints the endpoint every 5s so the user knows

    let print_endpoint_simple = || {
        let ip_add = get_ip_as_str();

        println!("\n opc.tcp://{}:{}{} \n",ip_add,4840,CUSTOM_ENDPOINT_PATH);
    };
    //server.add_polling_action(5000, print_endpoint);
    server.add_polling_action(5000, print_endpoint_simple);


    // now, for PRKE we have, initial conditions and timestep
    // note: for prompt criticality to work, timestep must be smaller or equal to 
    // the neutron generation time, otherwise, the solution to the matrices 
    // causes the values to become negative
    let timestep = Time::new::<uom::si::time::millisecond>(2.0);

    let prke_six_group = SixGroupPRKE::default();
    let prke_six_group_ptr = Arc::new(Mutex::new(prke_six_group));

    // not only that, we need to get the fuel_temperature_feedback_struct
    // set initial temperature to 300K
    let mut fuel_temperature_feedback_struct = 
        SimpleFuelTemperatureFeedback::default();

    let reference_temperature = ThermodynamicTemperature::new::<kelvin>(300.0);

    fuel_temperature_feedback_struct.set_fuel_temperature(
        ThermodynamicTemperature::new::<kelvin>(300.0)).unwrap();

    let fuel_temperature_feedback_struct_ptr = 
        Arc::new(Mutex::new(fuel_temperature_feedback_struct));

    let surrounding_temperature = 
        ThermodynamicTemperature::new::<kelvin>(293.0);

    // macroscopic fission XS 
    let macroscopic_fission_xs: LinearNumberDensity = 
        LinearNumberDensity::new::<per_meter>(0.1);


    // control rod properties 
    // typical values taken from lamarsh
    let rod_worth: Ratio = Ratio::new::<ratio>(0.007);
    let cylinder_height: Length = Length::new::<centimeter>(83.0);

    // baseline reactivity 

    let baseline_excess_reactivity: Ratio = 
        Ratio::new::<ratio>(0.0020);


    

    // clone the ptr to move into the loop 
    let prke_six_group_ptr_clone_for_loop = prke_six_group_ptr.clone();
    let fuel_temperature_feedback_struct_ptr_clone_for_loop 
        = fuel_temperature_feedback_struct_ptr.clone();

    // timer
    let loop_time = SystemTime::now();
    // neutron mean lifetime in thermal spectrum reactor is about 10^(-4)s
    let neutron_mean_lifetime = Time::new::<microsecond>(100.0);
    let background_source_rate = VolumetricNumberRate::new::<per_cubic_meter_second>(5.0);

    let prke_loop = move ||{
        // timer start 
        let loop_time_start = loop_time.elapsed().unwrap();


        let neutron_and_precursor_conc: Array1<VolumetricNumberDensity>;

        // now we have calculation steps, we need to read reactivity 
        // from the user input first
        let reactor_power_kilowatts: f64;

        {
            let mut address_space_lock = address_space.write();
            let control_rod_insertion_length_set_point: f64 = address_space_lock.
                get_variable_value(
                    control_rod_set_point_node_cm.clone())
                .unwrap().value.unwrap()
                .as_f64().unwrap();

            let insertion_length: Length = 
                Length::new::<centimeter>(control_rod_insertion_length_set_point);

            // once we get control rod insertion set point, we convert it 
            // into a reactivity 
            let control_rod_reactivity: Ratio = 
                obtain_rod_worth_cylinder(
                    cylinder_height, 
                    insertion_length, 
                    rod_worth).unwrap();


            // now find the fuel temperature reactivity
            // obtain the prke lock, perform the calculations based 
            // on reactivity
            let mut prke_lock_deref_ptr = prke_six_group_ptr_clone_for_loop.lock().unwrap();
            let mut fuel_temperature_feedback_struct_deref_ptr = 
                fuel_temperature_feedback_struct_ptr_clone_for_loop.lock().unwrap();

            let fuel_temperature_reactivity: Ratio = {

                // first let's find the fission power 
                //
                // fission rate is Sigma_f phi
                //
                // phi is n * v
                let neutron_conc: VolumetricNumberDensity 
                    = prke_lock_deref_ptr.deref_mut().get_current_neutron_population();

                let thermal_neutron_speed = Velocity::new::<meter_per_second>(2200.0);

                let neutron_flux = neutron_conc * thermal_neutron_speed;

                let fission_rate_per_unit_vol: VolumetricNumberRate =  
                    (neutron_flux * macroscopic_fission_xs).into();

                // now we have a fission rate per unit volume, we need to 
                // get fission power 
                // which we assume is 180 MeV 

                let energy_per_fission: Energy = 
                    Energy::new::<megaelectronvolt>(180.0);

                let fuel_volume = fuel_temperature_feedback_struct_deref_ptr.fuel_volume;

                let fission_power: Power = fission_rate_per_unit_vol * 
                    energy_per_fission * 
                    fuel_volume;

                // record reactor power 
                reactor_power_kilowatts = fission_power.get::<kilowatt>();


                // add fission power, and remove convection heat

                fuel_temperature_feedback_struct_deref_ptr.add_fission_heat(
                    fission_power, timestep).unwrap();

                let coolant_temperature = surrounding_temperature;
                fuel_temperature_feedback_struct_deref_ptr.remove_convection_heat(
                    coolant_temperature, timestep).unwrap();

                // now get reactivity value 

                let fuel_temp_reactivity_value: Ratio = 
                    fuel_temperature_feedback_struct_deref_ptr.
                    obtain_fuel_temperature_delta_rho(reference_temperature)
                    .unwrap();

                // return reactivity 
                fuel_temp_reactivity_value


            };
            


            let reactivity = baseline_excess_reactivity + 
                fuel_temperature_reactivity +
                control_rod_reactivity;


            let keff = SixGroupPRKE::get_keff_from_reactivity(reactivity);
            let neutron_generation_time: Time = neutron_mean_lifetime/keff;




            // check if neutron pop is too large 
            // otherwise simulator goes to infinity
            let max_neutron_conc = VolumetricNumberDensity::new::<per_cubic_meter>(
                (0.01*f64::MAX).into());

            let current_neutron_conc: VolumetricNumberDensity 
                = prke_lock_deref_ptr.get_current_neutron_population();

            //dbg!(&current_neutron_conc);
            //dbg!(&max_neutron_conc);

            if current_neutron_conc > max_neutron_conc {

                let now = DateTime::now();
                // rod drop if neutron concentration too high
                let _ = address_space_lock.set_variable_value(
                    control_rod_set_point_node_cm.clone(), 
                    -0.05 as f64,
                    &now, 
                    &now);
            }

            neutron_and_precursor_conc = prke_lock_deref_ptr
                .deref_mut()
                .solve_next_timestep_precursor_concentration_and_neutron_pop_vector(
                    timestep, 
                    reactivity, 
                    neutron_generation_time, 
                    background_source_rate).unwrap();
            // after we get new precursor concentration, we can move on to 
            // informing the client of these details


            // get reactivity in dollars
            let delayed_fraction = prke_lock_deref_ptr.deref().get_total_delayed_fraction();
            let reactivity_dollars = reactivity/delayed_fraction;

            //dbg!(&reactivity_dollars);


        }

        // for writing values to server and sending to client
        // postprocessing, print out neutron pop
        {
            let neutron_conc: VolumetricNumberDensity = 
                neutron_and_precursor_conc[0];
            let precursor_1_conc: VolumetricNumberDensity = 
                neutron_and_precursor_conc[1];
            let precursor_2_conc: VolumetricNumberDensity = 
                neutron_and_precursor_conc[2];
            let precursor_3_conc: VolumetricNumberDensity = 
                neutron_and_precursor_conc[3];
            let precursor_4_conc: VolumetricNumberDensity = 
                neutron_and_precursor_conc[4];
            let precursor_5_conc: VolumetricNumberDensity = 
                neutron_and_precursor_conc[5];
            let precursor_6_conc: VolumetricNumberDensity = 
                neutron_and_precursor_conc[6];

            // get neutron conc in per m3 rounded to 1
            // decimal place
            let neutron_conc_per_m3: f64 = 
            (neutron_conc.get::<per_cubic_meter>()*10.0)
            .round() / 10.0;
            // or not rounded
            let neutron_conc_per_m3: f64 = neutron_conc.get::<per_cubic_meter>() ;

            // set neutron conc node
            let mut address_space_lock = address_space.write();
            let now = DateTime::now();
            let _ = address_space_lock.set_variable_value(
                neutron_concentration_node.clone(), 
                neutron_conc_per_m3 as f64,
                &now, 
                &now);

            // now for temperature 
            let current_fuel_temperature: ThermodynamicTemperature = 
                fuel_temperature_feedback_struct_ptr_clone_for_loop
                .lock()
                .unwrap()
                .get_fuel_temperature()
                .unwrap();

            let fuel_temp_deg_c: f64 = 
                current_fuel_temperature.get::<degree_celsius>();
            let _ = address_space_lock.set_variable_value(
                fuel_temperature_node.clone(), 
                fuel_temp_deg_c as f64,
                &now, 
                &now);
            // write reactor power
            let _ = address_space_lock.set_variable_value(
                reactor_power_node.clone(), 
                reactor_power_kilowatts as f64,
                &now, 
                &now);

            // deal with precursors later

            //dbg!(&neutron_conc_per_m3);
            //dbg!(&precursor_3_conc);
        }

        let time_taken_for_calculation_loop = loop_time.elapsed().unwrap()
        - loop_time_start;
        //dbg!(&time_taken_for_calculation_loop);

    };

    server.add_polling_action(
        timestep.get::<uom::si::time::millisecond>().round() as u64, 
        prke_loop);

    //server.add_polling_action(
    //    timestep.get::<uom::si::time::millisecond>().round() as u64, 
    //    prke_loop);

    if run_server { server.run(); }

}

fn get_keff_from_reactivity(reactivity: Ratio) -> Ratio {

    // reactivity is rho 
    //
    // rho = (k-1)/k
    //
    // k * rho = k - 1
    // k * rho - k = - 1
    // k - k * rho = 1
    // k * (1 - rho) = 1
    // k = 1/(1 - rho) 
    //

    let ratio_one = Ratio::new::<ratio>(1.0);

    let keff = ratio_one/(ratio_one - reactivity);

    keff

}

const CUSTOM_ENDPOINT_PATH: &str = "/rust_fuel_temp_control_rod_prke_opcua_server";

fn build_standard_server() -> Server {

    let server_builder = ServerBuilder::new();

    let server_builder = 
        server_builder.application_name("test server_builder");

    let server_builder =
        server_builder.application_uri("urn:OPC UA Sample Server");




    let ip_address = get_ip_as_str();

    let server_builder = 
        server_builder.host_and_port(&ip_address, 4840);


    let server_builder =
        server_builder.discovery_urls(
            vec![
            CUSTOM_ENDPOINT_PATH.into(),
            ]);


    // username and password is just anonymous

    let user_id_anonymous = config::ANONYMOUS_USER_TOKEN_ID;


    let user_id_vector = 
        vec![user_id_anonymous]
        .iter()
        .map(|u| u.to_string())
        .collect::<Vec<String>>();




    let path = CUSTOM_ENDPOINT_PATH;


    let my_endpoints = vec![
        ("custom_path", ServerEndpoint::new_none(path,&user_id_vector)),
    ];


    let server_builder = 
        server_builder.endpoints(my_endpoints);

    // then we build the server

    let server = server_builder.server().unwrap();
    return server;

}

fn get_ip_as_str() -> String {

    let my_local_ip = local_ip().unwrap();

    // i can convert it to a string

    let ip_add_string : String = my_local_ip.to_string();

    return ip_add_string;

}
