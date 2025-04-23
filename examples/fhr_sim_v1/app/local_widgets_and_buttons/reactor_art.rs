use std::f32::consts::TAU;

use egui::{vec2, Color32, Sense, Stroke, Ui, Vec2};

// first thing, reactor
//
// it needs a top left coordinate first
// these will make the rectangle
pub fn fhr_reactor_vessel(ui: &mut Ui,
    rectangle: egui::Rect){

    // make a new painter first 
    //


    // top_left
    let left_most_side = rectangle.left();
    let top_most_side = rectangle.top();
    let right_most_side = rectangle.right();
    let bottom_most_side = rectangle.bottom();

    let breadth = (left_most_side - right_most_side).abs();
    let height = (top_most_side - bottom_most_side).abs();

    let size = Vec2::new(breadth, height);

    let (response, painter) = ui.allocate_painter(
        size, Sense::hover()
    );
    let rect = response.rect;
    let c = rect.center();
    let r = rect.width() / 2.0 - 1.0;
    let color = Color32::from_gray(128);
    let stroke = Stroke::new(1.0, color);
    painter.circle_stroke(c, r, stroke);
    painter.line_segment([c - vec2(0.0, r), c + vec2(0.0, r)], stroke);
    painter.line_segment([c, c + r * Vec2::angled(TAU * 1.0 / 8.0)], stroke);
    painter.line_segment([c, c + r * Vec2::angled(TAU * 3.0 / 8.0)], stroke);


}

