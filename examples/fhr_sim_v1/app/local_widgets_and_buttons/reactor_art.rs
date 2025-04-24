use std::f32::consts::TAU;

use egui::{epaint::PathShape, vec2, Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};

use super::hot_to_cold_colour_mark_1;

// first thing, reactor
//
// it needs a top left coordinate first
// these will make the rectangle
//
// i think for easy gui connections, I can start making classes 
// and stuff for easy connection
pub fn fhr_reactor_vessel(ui: &mut Ui,
    user_rect: egui::Rect){

    // make a new painter first 
    //


    // top_left
    let left_most_side = user_rect.left();
    let top_most_side = user_rect.top();
    let right_most_side = user_rect.right();
    let bottom_most_side = user_rect.bottom();

    let max_height_y = (top_most_side - bottom_most_side).abs();
    let max_width_x = (left_most_side - right_most_side).abs();

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

    let reactor_half_length_y = max_height_y * 0.5 * 0.8;
    let reactor_half_width_x = max_width_x * 0.5 * 0.8;
    //painter.circle_stroke(c, r, stroke);
    
    // now let's paint the reactor first
    painter.line_segment(
        [c + vec2(reactor_half_width_x, reactor_half_length_y), 
        c + vec2(reactor_half_width_x, -reactor_half_length_y)], 
        stroke
    );
    painter.line_segment(
        [c + vec2(-reactor_half_width_x, reactor_half_length_y), 
        c + vec2(-reactor_half_width_x, -reactor_half_length_y)], 
        stroke
    );
    //painter.line_segment([c, c + r * Vec2::angled(TAU * 1.0 / 8.0)], stroke);
    //painter.line_segment([c, c + r * Vec2::angled(TAU * 3.0 / 8.0)], stroke);
    //
    // if wanting filled shapes:
    // https://docs.rs/epaint/0.31.1/epaint/struct.PathShape.html
    //
    // also Beizier curve
    // https://docs.rs/epaint/0.31.1/epaint/struct.QuadraticBezierShape.html
    // https://docs.rs/epaint/0.31.1/epaint/struct.CubicBezierShape.html
    //
    // with these filled curves, I can do a shape which 
    // for which I can then use the add method to add shapes 
    // to the painter:
    // 
    // https://docs.rs/egui/latest/egui/struct.Painter.html
    //
    // so to get the shape, i have a complex polygon
    // The points are

    

    // bottom inlet
    let fhr_coolant_inlet_bottom_left = c + vec2(-0.10*reactor_half_width_x, reactor_half_length_y* 0.9);
    let fhr_coolant_inlet_bottom_right = c + vec2(0.10*reactor_half_width_x, reactor_half_length_y* 0.9);

    // core part
    let fhr_core_inlet_bottom_left = c + vec2(-0.10*reactor_half_width_x, reactor_half_length_y* 0.75);
    let fhr_core_inlet_bottom_right = c + vec2(0.10*reactor_half_width_x, reactor_half_length_y* 0.75);
    let fhr_core_fat_bottom_left = c + vec2(-0.50*reactor_half_width_x, reactor_half_length_y* 0.55);
    let fhr_core_fat_bottom_right = c + vec2(0.50*reactor_half_width_x, reactor_half_length_y* 0.55);

    let fhr_core_fat_top_left = c + vec2(-0.50*reactor_half_width_x, -reactor_half_length_y* 0.55);
    let fhr_core_fat_top_right = c + vec2(0.50*reactor_half_width_x, -reactor_half_length_y* 0.55);
    let fhr_core_outlet_top_left = c + vec2(-0.10*reactor_half_width_x, -reactor_half_length_y* 0.75);
    let fhr_core_outlet_top_right = c + vec2(0.10*reactor_half_width_x, -reactor_half_length_y* 0.75);

    // top outlet
    let fhr_coolant_outlet_top_left = c + vec2(-0.10*reactor_half_width_x, -reactor_half_length_y* 0.9);
    let fhr_coolant_outlet_top_right = c + vec2(0.10*reactor_half_width_x, -reactor_half_length_y* 0.9);

    // colour fill
    let hotness: f32 = 0.1;
    let fill = hot_to_cold_colour_mark_1(hotness);

    // draw clockwise
    let core_bottom_points = vec![
        //fhr_coolant_inlet_bottom_left,
        fhr_core_inlet_bottom_left,
        fhr_core_fat_bottom_left,
        //fhr_core_fat_top_left,
        //fhr_core_outlet_top_left,
        //fhr_coolant_outlet_top_left,
        //fhr_coolant_outlet_top_right,
        //fhr_core_outlet_top_right,
        //fhr_core_fat_top_right,
        fhr_core_fat_bottom_right,
        fhr_core_inlet_bottom_right,
        //fhr_coolant_inlet_bottom_right,
    ];
    let core_bottom_inlet_points = vec![
        fhr_coolant_inlet_bottom_left,
        fhr_core_inlet_bottom_left,
        //fhr_core_fat_bottom_left,
        //fhr_core_fat_top_left,
        //fhr_core_outlet_top_left,
        //fhr_coolant_outlet_top_left,
        //fhr_coolant_outlet_top_right,
        //fhr_core_outlet_top_right,
        //fhr_core_fat_top_right,
        //fhr_core_fat_bottom_right,
        fhr_core_inlet_bottom_right,
        fhr_coolant_inlet_bottom_right,
    ];
    let core_mid_points = vec![
        //fhr_coolant_inlet_bottom_left,
        //fhr_core_inlet_bottom_left,
        fhr_core_fat_bottom_left,
        fhr_core_fat_top_left,
        //fhr_core_outlet_top_left,
        //fhr_coolant_outlet_top_left,
        //fhr_coolant_outlet_top_right,
        //fhr_core_outlet_top_right,
        fhr_core_fat_top_right,
        fhr_core_fat_bottom_right,
        //fhr_core_inlet_bottom_right,
        //fhr_coolant_inlet_bottom_right,
    ];
    
    // fhr coolant 
    let fhr_core_bottom_coolant_shape = 
        PathShape::convex_polygon(core_bottom_points, fill, stroke);
    let fhr_core_inlet_coolant_shape = 
        PathShape::convex_polygon(core_bottom_inlet_points, fill, stroke);
    let fhr_core_mid_coolant_shape = 
        PathShape::convex_polygon(core_mid_points, fill, stroke);

    painter.add(fhr_core_bottom_coolant_shape);
    painter.add(fhr_core_inlet_coolant_shape);
    painter.add(fhr_core_mid_coolant_shape);

}

