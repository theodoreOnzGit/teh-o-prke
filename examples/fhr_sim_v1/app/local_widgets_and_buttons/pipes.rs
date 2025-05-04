use egui::{epaint::{CubicBezierShape, PathShape}, vec2, Color32, Pos2, Sense, Stroke, Vec2, Widget};
use uom::si::{f64::*, thermodynamic_temperature::degree_celsius};


use super::hot_to_cold_colour_mark_1;

pub struct SinglePipe {
    size: Vec2,
    min_temp: ThermodynamicTemperature,
    max_temp: ThermodynamicTemperature,
    temp: ThermodynamicTemperature,
}

impl SinglePipe {

    pub fn new(size: Vec2,
        min_temp: ThermodynamicTemperature,
        max_temp: ThermodynamicTemperature,
        temp: ThermodynamicTemperature,) -> Self {

        Self { size, 
            min_temp, 
            max_temp, 
            temp,
        }

    }


    /// returns hotness based on max and min temp of fhr 
    pub fn hotness(&self, temp: ThermodynamicTemperature) -> f32 {

        let button_temp_degc = temp.get::<degree_celsius>();
        let min_temp_degc = self.min_temp.get::<degree_celsius>();
        let max_temp_degc = self.max_temp.get::<degree_celsius>();

        let hotness: f64 = 
            (button_temp_degc - min_temp_degc)/(max_temp_degc- min_temp_degc);

        return hotness as f32;
    }
    /// gets the size of the widget 
    pub fn size(&self) -> Vec2 {

        self.size.clone()
    }
}

impl Widget for SinglePipe {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {

        let size = self.size();
        let (response, painter) = ui.allocate_painter(
            size, Sense::hover()
        );
        let pipe_hotness = 
            self.hotness(self.temp);

        let pipe_colour = hot_to_cold_colour_mark_1(
            pipe_hotness
        );
        // let colour = 
        let width = 20.0;

        let stroke = Stroke::new(width, pipe_colour);

        // get coordinates based on center
        let rect = response.rect;
        let pipe_ctr = rect.center();

        let delta_x = rect.width();
        let delta_y = rect.height();

        painter.line_segment(
            [pipe_ctr - vec2(0.50*delta_x, 0.50*delta_y), 
            pipe_ctr + vec2(0.50*delta_x, 0.50*delta_y)], 
            stroke
        );
        response
    }
}
