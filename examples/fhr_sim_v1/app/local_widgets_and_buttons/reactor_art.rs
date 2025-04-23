use egui::{Painter, Pos2, Sense, Ui, Vec2};

// first thing, reactor
//
// it needs a top left coordinate first
// these will make the rectangle
pub fn fhr_reactor_vessel(ui: &mut Ui,
    rectangle: egui::Rect){

    // make a new painter first 
    //

    let arbitrary_size = Vec2::splat(16.0);

    // top_left
    let rectangle_min: Pos2 = rectangle.min;

    let (response, painter) = ui.allocate_painter(
        arbitrary_size, Sense::hover()
    );


}

