use egui::{Painter, Pos2, Sense, Ui, Vec2};

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


}

