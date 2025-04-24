use std::f32::consts::TAU;

use egui::{vec2, Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};

// first thing, reactor
//
// it needs a top left coordinate first
// these will make the rectangle
pub fn fhr_reactor_vessel(ui: &mut Ui,
    user_rect: egui::Rect){

    // make a new painter first 
    //


    // top_left
    let left_most_side = user_rect.left();
    let top_most_side = user_rect.top();
    let right_most_side = user_rect.right();
    let bottom_most_side = user_rect.bottom();

    let max_height = (top_most_side - bottom_most_side).abs();
    let max_width = (left_most_side - right_most_side).abs();

    let ui_rectangle: Rect = ui.min_rect();
    let breadth = ui_rectangle.right();
    let height = ui_rectangle.bottom();

    // the size here is the size of the painter
    let size = Vec2::new(breadth, height);

    let (response, painter) = ui.allocate_painter(
        size, Sense::hover()
    );
    let response_rect = response.rect;
    
    // what I want to do now is to shift the 
    // response rectangle

    let shift: Pos2 = user_rect.min;

    // the rect here is 
    let rect: egui::Rect = 
        egui:: Rect {
            min: user_rect.min,
            max: user_rect.max,
        };
    
    let c = rect.center();
    // circle radius is r
    let r = rect.width() / 2.0 - 1.0;
    let color = Color32::from_gray(128);
    let stroke = Stroke::new(1.0, color);

    let reactor_half_length = max_height * 0.5 * 0.8;
    let reactor_half_width = max_width * 0.5 * 0.8;
    //painter.circle_stroke(c, r, stroke);
    
    // now let's paint the reactor first
    painter.line_segment(
        [c + vec2(reactor_half_width, reactor_half_length), 
        c + vec2(reactor_half_width, -reactor_half_length)], 
        stroke
    );
    painter.line_segment(
        [c + vec2(-reactor_half_width, reactor_half_length), 
        c + vec2(-reactor_half_width, -reactor_half_length)], 
        stroke
    );
    //painter.line_segment([c, c + r * Vec2::angled(TAU * 1.0 / 8.0)], stroke);
    //painter.line_segment([c, c + r * Vec2::angled(TAU * 3.0 / 8.0)], stroke);


}

