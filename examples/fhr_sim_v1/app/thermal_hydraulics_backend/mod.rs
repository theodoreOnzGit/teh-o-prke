use std::sync::{Arc, Mutex};

use std::thread;
use std::time::{Duration, SystemTime};

use components::*;
//use teh_o_prke::decay_heat::DecayHeat;
//use teh_o_prke::feedback_mechanisms::fission_product_poisons::Xenon135Poisoning;
//use teh_o_prke::zero_power_prke::six_group::FissioningNuclideType;
//use teh_o_prke::{feedback_mechanisms::SixFactorFormulaFeedback, zero_power_prke::six_group::SixGroupPRKE};
//use uom::si::area::square_meter;
//use uom::si::energy::{kilojoule, megaelectronvolt};
//use uom::si::heat_transfer::watt_per_square_meter_kelvin;
//use uom::si::linear_number_density::per_meter;
//use uom::si::mass::kilogram;
//use uom::si::power::megawatt;
use uom::si::time::{microsecond, second};
//use uom::si::velocity::meter_per_second;
//use uom::si::volume::cubic_meter;
//use uom::si::volumetric_number_rate::per_cubic_meter_second;
use uom::si::f64::*;
//use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::ConstZero;


use crate::{FHRSimulatorApp, FHRState};

impl FHRSimulatorApp {
    pub fn calculate_thermal_hydraulics_loop(
        fhr_state: Arc<Mutex<FHRState>>){

        let thermal_hydraulics_timestep = Time::new::<second>(0.1);

        let fhr_state_clone = fhr_state.clone();
        // now, time controls 
        let loop_time = SystemTime::now();
        let mut current_simulation_time = Time::ZERO;

        // create components first
        let initial_temperature = ThermodynamicTemperature::new::<degree_celsius>(
            fhr_state_clone.lock().unwrap().core_outlet_temp_degc
        );
        let reactor_vessel_1 = new_reactor_vessel_pipe_1(initial_temperature);
        let downcomer_pipe_2 = new_downcomer_pipe_2(initial_temperature);
        let downcomer_pipe_3 = new_downcomer_pipe_3(initial_temperature);
        let pipe_4 = new_fhr_pipe_4(initial_temperature);
        let pri_loop_pump = new_fhr_pri_loop_pump(initial_temperature);

        // create initial mass flowrates 

        let mut pri_loop_forced_circ_mass_flowrate = MassRate::ZERO;
        let mut core_mass_flowrate = MassRate::ZERO;
        let mut downcomer_pipe_2_mass_flowrate = MassRate::ZERO;
        let mut downcomer_pipe_3_mass_flowrate = MassRate::ZERO;


        // calculation loop (indefinite)
        //
        // to be done once every timestep
        loop {

            // so now, let's do the necessary things
            // first, timestep and loop time 
            //
            // second, read and update the local_ciet_state

            let loop_time_start = loop_time.elapsed().unwrap();

            calculate_thermal_hydraulics_for_one_timestep(
                &mut fhr_state_clone.lock().unwrap(),
                thermal_hydraulics_timestep,
            );

            current_simulation_time += thermal_hydraulics_timestep;

            let simulation_time_seconds = current_simulation_time.get::<second>();

            let elapsed_time_seconds = 
                (loop_time.elapsed().unwrap().as_secs_f64() * 100.0).round()/100.0;

            let overall_simulation_in_realtime_or_faster: bool = 
                simulation_time_seconds > elapsed_time_seconds;

            // now update the ciet state 
            let loop_time_end = loop_time.elapsed().unwrap();
            let time_taken_for_calculation_loop_microseconds: f64 = 
                (loop_time_end - loop_time_start)
                .as_micros() as f64;



            let time_to_sleep_microseconds: u64 = 
                (thermal_hydraulics_timestep.get::<microsecond>() - 
                 time_taken_for_calculation_loop_microseconds)
                .round().abs() as u64;

            let time_to_sleep: Duration = 
                Duration::from_micros(time_to_sleep_microseconds - 1);


            // last condition for sleeping
            let real_time_in_current_timestep: bool = 
                time_to_sleep_microseconds > 1;

            //
            let fast_forward_botton_on = false;

            if overall_simulation_in_realtime_or_faster && 
                real_time_in_current_timestep && 
                    !fast_forward_botton_on 
            {
                thread::sleep(time_to_sleep);
            } else if overall_simulation_in_realtime_or_faster 
                && real_time_in_current_timestep 
                    && fast_forward_botton_on 
            {
                // sleep 5 microseconds if fast fwd
                let short_time_to_sleep: Duration = Duration::from_micros(5);
                thread::sleep(short_time_to_sleep);
            } else {
                // don't sleep

            }
            //let time_to_sleep = Duration::from_millis(40);

            //dbg!(&(
            //        time_taken_for_calculation_loop_microseconds,
            //        prke_timestep.get::<microsecond>(),
            //)
            //);
            //thread::sleep(time_to_sleep);
        }

        #[inline]
        pub fn calculate_thermal_hydraulics_for_one_timestep(
            fhr_state_ref: &mut FHRState,
            thermal_hydraulics_timestep: Time,
        ){

            // over here, I will have four parallel branches in the 
            // main loop 
            //
            // two deal with downcomer
            // one with the main core 
            // and the last one is the main outside loop with 
            // the pump
            //
            // the left and right downcomer can be represented with  
            // one vertical pipe each for simplicity 
            // the core part can be represented with 
            // one vertical pipe too 
            //
            // the outside loop can be represented with two  
            // horizontal pipes and one vertical pipe 
            //
            // there will be no DRACS loop, as heat is removed via 
            // radiation. 
            // In the ARVACS system for KP-FHR it is removed to ambient 
            // surroundings through use of radiant heat transfer.
            //
            // these then transfer heat to thin thimbles containing water 
            // which boils
            //
            // this can be heat transferred to some outside boundary condition
            //
            // now, just like CIET, we have a top and bottom mixing node

            let _ = fhr_state_ref;
            let _ = thermal_hydraulics_timestep;
        }

    }
}

/// contains simple components for the fhr simulator
///
/// these are components for primary loop and secondary loop 
/// turbine components not included (will be in tampines-steam-tables)
pub mod components;

/// contains functions for calculating pri loop 
/// fluid mechanics
pub mod pri_loop_fluid_mechanics_calc_fns;
