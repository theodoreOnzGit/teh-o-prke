use std::{sync::{Arc, Mutex}, thread};


/// this represents the first iteration 
/// of the fhr simulator
///
/// basically one can do a FHR loop 
/// with a permenantly steady state steam cycle
/// the latter uses the tampines-steam-tables
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
fn main(){

    fhr_simulator_v1().unwrap();


}
pub fn fhr_simulator_v1() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "FHR Simulator V1 Powered by TUAS and teh-o-prke",
        native_options,
        Box::new(|cc| {
            // image support,
            // from 
            // https://github.com/emilk/egui/tree/master/examples/images
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(FHRSimulatorApp::new(cc)))

    }

        ),
    )
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Clone, Debug)]
pub struct FHRSimulatorApp {

    pub fhr_state: Arc<Mutex<FHRState>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[derive(Clone,Copy, Debug)]
pub struct FHRState {
    /// left control rod insertion fraction
    pub left_cr_insertion_frac: f32,
    /// right control rod insertion fraction
    pub right_cr_insertion_frac: f32,

    // temperatures for both reactor feedback and display
    pub pebble_core_temp_degc: f64,
    pub pebble_bed_coolant_temp_degc: f64,
    pub core_bottom_temp_degc: f64,
    pub core_top_temp_degc: f64,
    pub core_inlet_temp_degc: f64,
    pub core_outlet_temp_degc: f64,
    pub left_downcomer_upper_temp_degc: f64,
    pub left_downcomer_mid_temp_degc: f64,
    pub left_downcomer_lower_temp_degc: f64,
    pub right_downcomer_upper_temp_degc: f64,
    pub right_downcomer_mid_temp_degc: f64,
    pub right_downcomer_lower_temp_degc: f64,
}

impl Default for FHRState {
    fn default() -> Self {
        let default_temperature_degc = 500.0;
        FHRState { 
            left_cr_insertion_frac: 1.0,
            right_cr_insertion_frac: 1.0,
            pebble_core_temp_degc: default_temperature_degc,
            pebble_bed_coolant_temp_degc: default_temperature_degc,
            core_bottom_temp_degc: default_temperature_degc,
            core_top_temp_degc: default_temperature_degc,
            core_inlet_temp_degc: default_temperature_degc,
            core_outlet_temp_degc: default_temperature_degc,
            left_downcomer_upper_temp_degc: default_temperature_degc,
            left_downcomer_mid_temp_degc: default_temperature_degc,
            left_downcomer_lower_temp_degc: default_temperature_degc,
            right_downcomer_upper_temp_degc: default_temperature_degc,
            right_downcomer_mid_temp_degc: default_temperature_degc,
            right_downcomer_lower_temp_degc: default_temperature_degc,

        }
    }
}


impl FHRSimulatorApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        //// Load previous app state (if any).
        //// Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        let new_fhr_app: FHRSimulatorApp = Default::default();

        let fhr_state_prke_ptr: Arc<Mutex<FHRState>> = 
            new_fhr_app.fhr_state.clone();
        let fhr_state_thermal_hydraulics_ptr: Arc<Mutex<FHRState>> = 
            new_fhr_app.fhr_state.clone();

        // now spawn a thread to do the kinetics
        //
        thread::spawn(move ||{
            FHRSimulatorApp::calculate_prke_loop(fhr_state_prke_ptr);
        });

        // spawn a thread to do the thermal hydraulics
        thread::spawn(move ||{
            fhr_state_thermal_hydraulics_ptr
        });

        new_fhr_app
    }

    
}
impl Default for FHRSimulatorApp {
    fn default() -> Self {

        let fhr_state = FHRState::default();
        let fhr_state_ptr = Arc::new(Mutex::new(fhr_state));

        Self {
            fhr_state: fhr_state_ptr
        }
    }
}

pub mod app;
