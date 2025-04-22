use std::thread;

/// this represents the first iteration 
/// of the fhr simulator
///
/// basically one can do a FHR loop 
/// with a permenantly steady state steam cycle
/// the latter uses the tampines-steam-tables
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
fn main(){



}
pub fn ciet_simulator_v1() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "CIET Simulator V1 Powered by TUAS",
        native_options,
        Box::new(|cc| {
            // image support,
            // from 
            // https://github.com/emilk/egui/tree/master/examples/images
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(CIETApp::new(cc)))

    }

        ),
    )
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CIETApp {

}
impl CIETApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        //// Load previous app state (if any).
        //// Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        let new_ciet_app: CIETApp = Default::default();

        // now spawn a thread moving in the pointer 
        //
        thread::spawn(move ||{

        });

        // spawn a thread to update the plotting bits
        thread::spawn(move ||{

        });

        new_ciet_app
    }

    
}
impl Default for CIETApp {
    fn default() -> Self {


        Self {


        }
    }
}
