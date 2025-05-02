use std::ops::Deref;

use egui::Ui;

use crate::{FHRSimulatorApp, FHRState};

use super::local_widgets_and_buttons::new_temp_sensitive_button;

impl FHRSimulatorApp {

    pub(crate) fn side_panel(&mut self, ui: &mut Ui){

        ui.heading("Fluoride Salt Cooled High Temperature Reactor (FHR) Controls");
        let mut fhr_state_ptr = self.fhr_state.lock().unwrap();

        let left_cr_slider = egui::Slider::new(
            &mut fhr_state_ptr.left_cr_insertion_frac, 
            0.0000..=1.0)
            .logarithmic(false)
            .text("Left Control Rod insertion Fraction")
            .drag_value_speed(0.001);

        ui.add(left_cr_slider);

        let right_cr_slider = egui::Slider::new(
            &mut fhr_state_ptr.right_cr_insertion_frac, 
            0.0000..=1.0)
            .logarithmic(false)
            .text("Right Control Rod insertion Fraction")
            .drag_value_speed(0.001);

        ui.add(right_cr_slider);

        // cloning the entire fhr state for diagnostics
        let fhr_state_clone: FHRState = fhr_state_ptr.deref().clone();
        //
        drop(fhr_state_ptr);

        ui.separator();
        ui.heading("FHR Diagnostics");

        let pebble_core_temp_degc = 
            fhr_state_clone.pebble_core_temp_degc;

        let pebble_bed_coolant_temp_degc = 
            fhr_state_clone.pebble_bed_coolant_temp_degc;

        // need pebble bed power and/or heat removal
        // and keff
        let keff = fhr_state_clone.keff;
        let reactor_power_megawatts = 
            fhr_state_clone.reactor_power_megawatts;

        ui.label("Reactor Power (MW-thermal):");
        ui.label(((1000.0*reactor_power_megawatts).round() / 1000.0).to_string());

        ui.label("Fuel Temperature Pebble Core/TRISO (deg C):");
        ui.label(((10.0*pebble_core_temp_degc).round() / 10.0).to_string());
        ui.label("Pebble Bed Coolant Temp (deg C):");
        ui.label(((10.0*pebble_bed_coolant_temp_degc).round() / 10.0).to_string());
        ui.label("k_eff");
        ui.label(((1.0e6*keff).round() / 1.0e6).to_string());


        // then temperature scale 

        ui.separator();
        ui.heading("Temperature Scale");
        ui.heading("Colour to Temperature Legend");

        // now I need colour legend
        let min_temp_degc = 450.0;
        let max_temp_degc = 1000.0;
        // max temp
        let button_temp_degc = max_temp_degc;
        let max_temp_string: String = 
            button_temp_degc.to_string()+" degC or more";
        let max_temp = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &max_temp_string
        );

        ui.add(max_temp);
        // 950.0
        let button_temp_degc = 950.0;
        let button_temp_string: String = 
            button_temp_degc.to_string()+" degrees celsius";
        let temp_950_degc = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &button_temp_string
        );
        ui.add(temp_950_degc);
        // 900.0
        let button_temp_degc = 900.0;
        let button_temp_string: String = 
            button_temp_degc.to_string()+" degrees celsius";
        let temp_900_degc = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &button_temp_string
        );
        ui.add(temp_900_degc);
        // 850.0
        let button_temp_degc = 850.0;
        let button_temp_string: String = 
            button_temp_degc.to_string()+" degrees celsius";
        let temp_850_degc = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &button_temp_string
        );
        ui.add(temp_850_degc);
        // 800.0
        let button_temp_degc = 800.0;
        let button_temp_string: String = 
            button_temp_degc.to_string()+" degrees celsius";
        let temp_800_degc = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &button_temp_string
        );
        ui.add(temp_800_degc);
        // 750.0
        let button_temp_degc = 750.0;
        let button_temp_string: String = 
            button_temp_degc.to_string()+" degrees celsius";
        let temp_750_degc = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &button_temp_string
        );
        ui.add(temp_750_degc);

        // 700.0
        let button_temp_degc = 700.0;
        let button_temp_string: String = 
            button_temp_degc.to_string()+" degrees celsius";
        let temp_700_degc = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &button_temp_string
        );
        ui.add(temp_700_degc);

        // 650.0
        let button_temp_degc = 650.0;
        let button_temp_string: String = 
            button_temp_degc.to_string()+" degrees celsius";
        let temp_650_degc = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &button_temp_string
        );
        ui.add(temp_650_degc);
        // 600.0
        let button_temp_degc = 600.0;
        let button_temp_string: String = 
            button_temp_degc.to_string()+" degrees celsius";
        let temp_600_degc = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &button_temp_string
        );
        ui.add(temp_600_degc);
        // 550.0
        let button_temp_degc = 550.0;
        let button_temp_string: String = 
            button_temp_degc.to_string()+" degrees celsius";
        let temp_550_degc = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &button_temp_string
        );
        ui.add(temp_550_degc);
        // 500.0
        let button_temp_degc = 500.0;
        let button_temp_string: String = 
            button_temp_degc.to_string()+" degrees celsius";
        let temp_500_degc = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &button_temp_string
        );
        ui.add(temp_500_degc);
        // 450.0
        let button_temp_degc = 450.0;
        let button_temp_string: String = 
            button_temp_degc.to_string()+" degrees celsius";
        let temp_450_degc = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &button_temp_string
        );
        ui.add(temp_450_degc);

        // 400.0
        let button_temp_degc = 400.0;
        let button_temp_string: String = 
            button_temp_degc.to_string()+" degrees celsius";
        let temp_400_degc = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &button_temp_string
        );
        ui.add(temp_400_degc);

        // 350.0
        let button_temp_degc = 350.0;
        let button_temp_string: String = 
            button_temp_degc.to_string()+" degrees celsius";
        let temp_350_degc = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &button_temp_string
        );
        ui.add(temp_350_degc);

        // then acknowledgements/citing
    }
}
