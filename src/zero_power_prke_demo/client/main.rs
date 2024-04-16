pub mod app;
//use std::ops::Deref;
use local_ip_address::local_ip;

pub use app::*;
use uom::si::frequency::hertz;

use crate::panels::{second_order_transfer_fn::SecondOrderStableTransferFn, decaying_sinusoid::DecayingSinusoid};
fn main() -> eframe::Result<()> {

    use core::time;
    use std::{thread, time::SystemTime, ops::DerefMut};
    use uom::si::{f64::*, time::{millisecond, second}};
    use crate::panels::opcua_panel::try_connect_to_server_and_run_client;
    use crate::first_order_transfer_fn::FirstOrderStableTransferFn;
    
    

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([400.0, 300.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };
    let gui_app = GuiClient::new();

    let plot_values_ptr_clone = gui_app.plot_points_ptr.clone();
    let rad_value_ptr_clone = gui_app.rad_value_ptr.clone();
    let time_now = SystemTime::now();

    // for input output plot
    let user_input_ptr_clone = gui_app.user_input.clone();
    let input_output_plots_ptr_clone = gui_app.input_output_plots_ptr.clone();

    // for opcua 

    let opcua_input_clone = gui_app.loop_pressure_drop_pump_pressure_pascals_input.clone();
    let opcua_output_clone = gui_app.mass_flowrate_kg_per_s_output.clone();
    let isothermal_ciet_plot_ptr_clone = gui_app.isothermal_ciet_plots_ptr.clone();
    let opcua_ip_addr_ptr_clone = gui_app.opcua_server_ip_addr.clone();

    let bt12_temp_deg_c_ptr_clone = gui_app.bt12_temp_deg_c.clone();
    let bt11_temp_deg_c_ptr_clone = gui_app.bt11_temp_deg_c.clone();
    let heater_power_kilowatts_ptr_clone = gui_app.heater_power_kilowatts.clone();
    let heater_v2_bare_ciet_plots_ptr_clone = gui_app.heater_v2_bare_ciet_plots_ptr.clone();

    // let's make a first order transfer fn 
    // G(s)
    let mut _g_s_first_order = FirstOrderStableTransferFn::new(
        1.0, 
        Time::new::<second>(1.0), 
        0.0, 
        0.0, 
        Time::new::<second>(4.0)
        );

    // this is for testing second order transfer fn 
    // G(s)
    let mut g_s_second_order_underdamped = SecondOrderStableTransferFn::new(
        1.0, // process gain
        Time::new::<second>(1.0),  // process time
        0.45, // damping factor
        0.0, 
        0.0, 
        Time::new::<second>(1.0)
    );

    let mut _g_s_second_order_crit_damped = SecondOrderStableTransferFn::new(
        1.0, // process gain
        Time::new::<second>(1.0),  // process time
        1.0, // damping factor
        0.0, 
        0.0, 
        Time::new::<second>(1.0)
    );

    let mut _g_s_second_order_over_damped = SecondOrderStableTransferFn::new(
        1.0, // process gain
        Time::new::<second>(1.0),  // process time
        2.15, // damping factor
        0.0, 
        0.0, 
        Time::new::<second>(1.0)
    );

    // decaying sinusoids 
    let mut g_s_decaying_sine = DecayingSinusoid::new_sine(
        1.0, 
        Frequency::new::<hertz>(0.5), 
        0.0, 
        0.0, 
        Time::new::<second>(1.0),
        Frequency::new::<hertz>(1.5), 
    );

    let mut _g_s_decaying_cosine = DecayingSinusoid::new_cosine(
        1.0, 
        Frequency::new::<hertz>(0.5), 
        0.0, 
        0.0, 
        Time::new::<second>(1.0),
        Frequency::new::<hertz>(1.5), 
    );
    // this is the thread for the user input and 
    // transfer fn
    thread::spawn(move||{
        loop {
            let time_elapsed_ms = time_now.elapsed().unwrap().as_millis();
            let time_elapsed_s: f64 = time_elapsed_ms as f64 / 1000 as f64;


            // push values to vecto64
            //
            //dbg!([time_elapsed_s,5.0]);
            let rad_value: f32 = 
                rad_value_ptr_clone.lock().unwrap().deref_mut().clone();

            plot_values_ptr_clone.lock().unwrap().deref_mut()
                .push([time_elapsed_s,rad_value as f64]);

            // user inputs and outputs must be editable in real-time and 
            // plotable
            let user_input: f32 = 
                user_input_ptr_clone.lock().unwrap().deref_mut().clone();


            let current_time = Time::new::<millisecond>(time_elapsed_ms as f64);


            let model_output_1 = g_s_decaying_sine.set_user_input_and_calc_output(
                current_time, user_input as f64);

            let model_output_2 = g_s_second_order_underdamped.set_user_input_and_calc_output(
                current_time, user_input as f64);
            
            let model_output = model_output_1 + model_output_2;

            //dbg!(&g_s_second_order_underdamped);
            //dbg!(&g_s_decaying_cosine);

            input_output_plots_ptr_clone.lock().unwrap().deref_mut()
                .push([time_elapsed_s,user_input as f64,
                model_output as f64]);

            thread::sleep(time::Duration::from_millis(100));
        }

    });

    // this is the portion where we do opc-ua

    // move client into the thread
    // plus the pointers
    thread::spawn(move || {

        // this is a simple connection loop, but doesn't reconnect 
        // if there is a disconnection

        let my_local_ip = local_ip().unwrap();
        let ip_addr: String = my_local_ip.to_string();        
        let endpoint: String = "opc.tcp://".to_owned()
        +&ip_addr+":4840/rust_ciet_opcua_server";

        let mut connection_result = try_connect_to_server_and_run_client(
            &endpoint,
            2,
            opcua_input_clone.clone(),
            opcua_output_clone.clone(),
            bt12_temp_deg_c_ptr_clone.clone(),
            bt11_temp_deg_c_ptr_clone.clone(),
            heater_power_kilowatts_ptr_clone.clone());

        // now, normally it should be well connected, if not, then 
        // retry 
        loop {

            let ip_addr: String = opcua_ip_addr_ptr_clone.lock().unwrap().deref_mut()
            .to_string();
            let endpoint: String = "opc.tcp://".to_owned()
            +&ip_addr+":4840/rust_ciet_opcua_server";

            if let Err(_) = connection_result.clone() {
                connection_result = try_connect_to_server_and_run_client(
                    &endpoint,
                    2,
                    opcua_input_clone.clone(),
                    opcua_output_clone.clone(),
                    bt12_temp_deg_c_ptr_clone.clone(),
                    bt11_temp_deg_c_ptr_clone.clone(),
                    heater_power_kilowatts_ptr_clone.clone());

            }

            let time_elapsed_ms = time_now.elapsed().unwrap().as_millis();
            let time_elapsed_s: f64 = time_elapsed_ms as f64 / 1000 as f64;

            let loop_pressure_drop_pascals: f32 = 
                opcua_input_clone.lock().unwrap().deref_mut().clone();
            let mass_flowrate_kg_per_s: f32 = 
                opcua_output_clone.lock().unwrap().deref_mut().clone();

            isothermal_ciet_plot_ptr_clone.lock().unwrap().deref_mut()
                .push([
                    time_elapsed_s,
                    loop_pressure_drop_pascals as f64,
                    mass_flowrate_kg_per_s as f64
                ]);

            let bt11_temp_deg_c: f32 = 
            bt11_temp_deg_c_ptr_clone.lock().unwrap().deref_mut().clone();
            let bt12_temp_deg_c: f32 = 
            bt12_temp_deg_c_ptr_clone.lock().unwrap().deref_mut().clone();
            let heater_power_kilowatts: f32 = 
            heater_power_kilowatts_ptr_clone.lock().unwrap().deref_mut().clone();
            
            heater_v2_bare_ciet_plots_ptr_clone.lock().unwrap().deref_mut()
                .push([
                    time_elapsed_s,
                    bt11_temp_deg_c as f64,
                    heater_power_kilowatts as f64,
                    bt12_temp_deg_c as f64,
                ]);


            

            thread::sleep(time::Duration::from_millis(100));
        }

        // now, if the client connects correctly, then we should be able 
        // to append the plots for the pointer

    });


    // last but not least, the main thread runs eframe natively
    eframe::run_native(
        "OPC-UA GUI Client",
        native_options,
        Box::new(|_cc| Box::new(gui_app)),
    )
}
