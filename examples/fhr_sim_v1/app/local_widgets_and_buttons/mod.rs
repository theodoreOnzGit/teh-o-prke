use crate::FHRSimulatorApp;

use egui::{Color32, Pos2, Rect, Ui, Widget};


impl FHRSimulatorApp {
    // places a widget at some area
    pub fn put_widget_with_size_and_centre(
        &mut self, ui: &mut Ui, widget: impl Widget,
        centre_x_pixels: f32,
        centre_y_pixels: f32,
        x_width_pixels: f32,
        y_width_pixels: f32){

        let top_left_x: f32 = centre_x_pixels - 0.5 * x_width_pixels;
        let top_left_y: f32 = centre_y_pixels - 0.5 * y_width_pixels;
        let bottom_right_x: f32 = centre_x_pixels + 0.5 * x_width_pixels;
        let bottom_right_y: f32 = centre_y_pixels + 0.5 * y_width_pixels;

        let rect: Rect = Rect {
            // top left
            min: Pos2 { x: top_left_x, y: top_left_y },
            // bottom right
            max: Pos2 { x: bottom_right_x, y: bottom_right_y },
        };

        ui.put(rect, widget);

    }

    pub fn place_vertical_widget_with_length(
        &mut self, ui: &mut Ui, widget: impl Widget,
        centre_x_pixels: f32,
        centre_y_pixels: f32,
        button_length: f32,
        aspect_ratio: f32,
        ){

        // aspect ratio is length by breadth (longer side by shorter side)
        
        let y_width_pixels = button_length;
        let mut x_width_pixels = button_length/aspect_ratio;

        // min width is 30 px 
        if x_width_pixels < 30.0 {
            x_width_pixels = 30.0;
        }

        self.put_widget_with_size_and_centre(
            ui, 
            widget, 
            centre_x_pixels, 
            centre_y_pixels, 
            x_width_pixels, 
            y_width_pixels);
    }

    pub fn place_horizontal_widget_with_length(
        &mut self, ui: &mut Ui, widget: impl Widget,
        centre_x_pixels: f32,
        centre_y_pixels: f32,
        button_length: f32,
        aspect_ratio: f32,
        ){

        // aspect ratio is length by breadth (longer side by shorter side)
        
        let x_width_pixels = button_length;
        let mut y_width_pixels = button_length/aspect_ratio;
        // min width is 30 px 
        if y_width_pixels < 30.0 {
            y_width_pixels = 30.0;
        }

        self.put_widget_with_size_and_centre(
            ui, 
            widget, 
            centre_x_pixels, 
            centre_y_pixels, 
            x_width_pixels, 
            y_width_pixels);
    }

    
}



pub fn new_temp_sensitive_button(
    min_temp_degc: f32, 
    max_temp_degc: f32,
    button_temp_degc: f32,
    name: &str,
) -> egui::Button {

    let hotness: f32 = 
        (button_temp_degc - min_temp_degc)/(max_temp_degc- min_temp_degc);

    let colour_temp = hot_to_cold_colour_mark_1(hotness);
    let temp_sensitive_button = egui::Button::new(name)
        .fill(colour_temp);

    temp_sensitive_button

}



/// From ChatGPT
/// Steps:
/// Cold colors (blue) start with high values in the blue channel (B = 1, G = 0).
/// Hot colors (red) end with high values in the red channel (R = 1, G = 0).
pub fn hot_to_cold_colour_mark_1(hotness: f32) -> Color32 {
    let mut hotness_clone = hotness.clone();

    // ensures hotness is between 0 and 1
    if hotness_clone < 0.0 {
        hotness_clone = 0.0;
    } else if hotness_clone > 1.0 {
        hotness_clone = 1.0
    }

    let red: f32 = 255.0 * hotness_clone;
    let green: f32 = 135.0 * (1.0 - hotness_clone);
    let blue: f32 = 255.0 * (1.0 - hotness_clone);

    return Color32::from_rgb(
        red as u8, 
        green as u8, 
        blue as u8);
}

pub mod reactor_art;

pub mod fhr_reactor_widget;
