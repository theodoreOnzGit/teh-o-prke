use std::time::Duration;

use egui::{vec2, CollapsingHeader, Color32, Pos2, Rect, Sense, Stroke, Vec2};
use local_widgets_and_buttons::reactor_art::fhr_reactor_vessel_prototype;

use crate::FHRSimulatorApp;

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
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FHR Educational Simulator v1");
            ui.separator();
            egui::menu::bar(ui, |ui| {

                egui::widgets::global_theme_preference_buttons(ui);
            });
            // for painting widgets
            // https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/misc_demo_window.rs
            //
            // the main thing is the painter class:
            // https://docs.rs/egui/latest/egui/struct.Painter.html
            //
            // here you can paint circles and rectangles 
            // images, line segments etc.

            CollapsingHeader::new("Misc")
                .default_open(false)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("You can pretty easily paint your own small icons:");
                        use std::f32::consts::TAU;
                        let size = Vec2::splat(16.0);
                        let (response, painter) = ui.allocate_painter(size, Sense::hover());
                        let rect = response.rect;
                        let c = rect.center();
                        let r = rect.width() / 2.0 - 1.0;
                        let color = Color32::from_gray(128);
                        let stroke = Stroke::new(1.0, color);
                        painter.circle_stroke(c, r, stroke);
                        painter.line_segment([c - vec2(0.0, r), c + vec2(0.0, r)], stroke);
                        painter.line_segment([c, c + r * Vec2::angled(TAU * 1.0 / 8.0)], stroke);
                        painter.line_segment([c, c + r * Vec2::angled(TAU * 3.0 / 8.0)], stroke);
                    });
                });



            //let size = egui::Vec2 { x: 150.0, y: 150.0 };

            //let tchx_pic = Image::new(
            //    include_image!("../../cooler.png")
            //    ).rounding(5.0).max_size(size);
            //ui.add(tchx_pic);

            // i want the UI top left... 

            let ui_rectangle: Rect = ui.min_rect();

            // this gives coordinates of top and left of the ui
            // for relative placement
            let left_most_side = ui_rectangle.left();
            let top_most_side = ui_rectangle.top();

            // next I want to have the reactor vessel 

            let reactor_offset_x: f32 = 120.0;
            let reactor_offset_y: f32 = 420.0;
            let reactor_height_px: f32 = 500.0;
            let reactor_width_px: f32 = 300.0;

            let reactor_rect_top_left: Pos2 = 
                Pos2 { 
                    x: left_most_side + reactor_offset_x, 
                    y:  top_most_side + reactor_offset_y
                };
            let reactor_rect_bottom_right: Pos2 = 
                Pos2 { 
                    x: reactor_rect_top_left.x + reactor_width_px, 
                    y: reactor_rect_top_left.y + reactor_height_px
                };

            let reactor_rectangle: egui::Rect =
                egui::Rect{
                    min: reactor_rect_top_left,
                    max: reactor_rect_bottom_right,
                };

            let control_rod_insertion_frac = 0.0;

            fhr_reactor_vessel_prototype(ui, reactor_rectangle,
                control_rod_insertion_frac);




            




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
