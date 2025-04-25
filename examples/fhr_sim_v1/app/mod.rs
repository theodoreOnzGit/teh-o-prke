use std::time::Duration;

use egui::{vec2, Pos2, Rect};
use local_widgets_and_buttons::fhr_reactor_widget::FHRReactorWidget;
use uom::si::f64::*;
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::{FHRSimulatorApp, FHRState};

impl eframe::App for FHRSimulatorApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui



        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        });

        egui::SidePanel::right("Supplementary Info").show(ctx, |ui|{
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

            //
            drop(fhr_state_ptr);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FHR Educational Simulator v1");
            ui.separator();
            egui::menu::bar(ui, |ui| {

                egui::widgets::global_theme_preference_buttons(ui);
            });
            let mut left_control_rod_insertion_frac 
                = 0.0;
            let mut right_control_rod_insertion_frac 
                = 0.0;

            egui::ScrollArea::both()
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                .drag_to_scroll(true)
                .show(ui, |ui| {

                    // for painting widgets
                    // https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/misc_demo_window.rs
                    //
                    // the main thing is the painter class:
                    // https://docs.rs/egui/latest/egui/struct.Painter.html
                    //
                    // here you can paint circles and rectangles 
                    // images, line segments etc.
                    // obtain lock first 

                    ui.separator();
                    // quickly clone the fhr state and drop ptr asap 
                    // just to read
                    let fhr_state_ptr = self.fhr_state.lock().unwrap();
                    let fhr_state_clone: FHRState = fhr_state_ptr.clone();
                    drop(fhr_state_ptr);

                    left_control_rod_insertion_frac 
                        = fhr_state_clone.left_cr_insertion_frac;
                    right_control_rod_insertion_frac 
                        = fhr_state_clone.right_cr_insertion_frac;
                    


                    let ui_rectangle: Rect = ui.min_rect();

                    // this gives coordinates of top and left of the ui
                    // for relative placement
                    let left_most_side = ui_rectangle.left();
                    let top_most_side = ui_rectangle.top();

                    let reactor_offset_x: f32 = 100.0;
                    let reactor_offset_y: f32 = 200.0;
                    let reactor_x_width_px: f32 = 150.0 * 1.5;
                    let reactor_y_height_px: f32 = 700.0 * 1.5;


                    let reactor_rect_top_left: Pos2 = 
                        Pos2 { 
                            x: left_most_side + reactor_offset_x, 
                            y: top_most_side + reactor_offset_y
                        };
                    let reactor_rect_bottom_right: Pos2 = 
                        Pos2 { 
                            x: reactor_rect_top_left.x + reactor_x_width_px, 
                            y: reactor_rect_top_left.y + reactor_y_height_px
                        };
                    let reactor_rectangle: egui::Rect =
                        egui::Rect{
                            min: reactor_rect_top_left,
                            max: reactor_rect_bottom_right,
                        };


                    let fhr_size = 
                        vec2(reactor_rectangle.width(), reactor_rectangle.height());

                    let min_temp = ThermodynamicTemperature::new::<degree_celsius>(450.0);
                    let max_temp = ThermodynamicTemperature::new::<degree_celsius>(800.0);
                    let pebble_core_temp = ThermodynamicTemperature::new::<degree_celsius>(
                        fhr_state_clone.pebble_core_temp_degc
                    );
                    let pebble_bed_coolant_temp = ThermodynamicTemperature::new::<degree_celsius>(
                        fhr_state_clone.pebble_bed_coolant_temp_degc
                    );
                    let core_bottom_temp = ThermodynamicTemperature::new::<degree_celsius>(
                        fhr_state_clone.core_bottom_temp_degc
                    );
                    let core_top_temp = ThermodynamicTemperature::new::<degree_celsius>(
                        fhr_state_clone.core_top_temp_degc
                    );
                    let core_inlet_temp = ThermodynamicTemperature::new::<degree_celsius>(
                        fhr_state_clone.core_inlet_temp_degc
                    );
                    let core_outlet_temp = ThermodynamicTemperature::new::<degree_celsius>(
                        fhr_state_clone.core_outlet_temp_degc
                    );
                    let left_downcomer_upper_temp = ThermodynamicTemperature::new::<degree_celsius>(
                        fhr_state_clone.left_downcomer_upper_temp_degc
                    );
                    let left_downcomer_mid_temp = ThermodynamicTemperature::new::<degree_celsius>(
                        fhr_state_clone.left_downcomer_mid_temp_degc
                    );
                    let left_downcomer_lower_temp = ThermodynamicTemperature::new::<degree_celsius>(
                        fhr_state_clone.left_downcomer_lower_temp_degc
                    );
                    let right_downcomer_upper_temp = ThermodynamicTemperature::new::<degree_celsius>(
                        fhr_state_clone.right_downcomer_upper_temp_degc
                    );
                    let right_downcomer_mid_temp = ThermodynamicTemperature::new::<degree_celsius>(
                        fhr_state_clone.right_downcomer_mid_temp_degc
                    );
                    let right_downcomer_lower_temp = ThermodynamicTemperature::new::<degree_celsius>(673.0);


                    let mut fhr_widget = FHRReactorWidget::new(
                        fhr_size,
                        min_temp,
                        max_temp,
                        pebble_core_temp,
                        pebble_bed_coolant_temp,
                        core_bottom_temp,
                        core_top_temp,
                        core_inlet_temp,
                        core_outlet_temp,
                        left_downcomer_upper_temp,
                        left_downcomer_mid_temp,
                        left_downcomer_lower_temp,
                        right_downcomer_upper_temp,
                        right_downcomer_mid_temp,
                        right_downcomer_lower_temp,
                    );
                    fhr_widget.set_left_cr_frac(left_control_rod_insertion_frac);
                    fhr_widget.set_right_cr_frac(right_control_rod_insertion_frac);

                    ui.put(reactor_rectangle, fhr_widget);

                    ui.separator();
                });













        });

        egui::TopBottomPanel::bottom("github").show(ctx, |ui|{

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });

        });

        


        ctx.request_repaint_after(Duration::from_millis(50));

        // adding the return here because there are too many closing 
        // parantheses
        // just demarcates the end
        return ();
    }
}
fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

pub mod local_widgets_and_buttons;
