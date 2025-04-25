use egui::{epaint::{CubicBezierShape, PathShape}, vec2, Color32, Pos2, Sense, Stroke, Vec2, Widget};
use uom::si::{f64::*, thermodynamic_temperature::degree_celsius};

use super::hot_to_cold_colour_mark_1;

pub struct FHRReactorWidget {
    size: Vec2,
    left_control_rod_insertion_frac: f32,
    right_control_rod_insertion_frac: f32,
    min_temp: ThermodynamicTemperature,
    max_temp: ThermodynamicTemperature,
    pebble_core_temp: ThermodynamicTemperature,
    core_mid_temp: ThermodynamicTemperature,
    core_bottom_temp: ThermodynamicTemperature,
    core_top_temp: ThermodynamicTemperature,
    core_inlet_temp: ThermodynamicTemperature,
    core_outlet_temp: ThermodynamicTemperature,
    left_downcomer_upper_temp: ThermodynamicTemperature,
    left_downcomer_mid_temp: ThermodynamicTemperature,
    left_downcomer_lower_temp: ThermodynamicTemperature,
    right_downcomer_upper_temp: ThermodynamicTemperature,
    right_downcomer_mid_temp: ThermodynamicTemperature,
    right_downcomer_lower_temp: ThermodynamicTemperature,
    
}


impl FHRReactorWidget {

    /// constructs a default FHRReactorWidget
    pub fn new(size: Vec2,
        min_temp: ThermodynamicTemperature,
        max_temp: ThermodynamicTemperature,
        pebble_core_temp: ThermodynamicTemperature,
        pebble_bed_coolant_temp: ThermodynamicTemperature,
        core_bottom_temp: ThermodynamicTemperature,
        core_top_temp: ThermodynamicTemperature,
        core_inlet_temp: ThermodynamicTemperature,
        core_outlet_temp: ThermodynamicTemperature,
        left_downcomer_upper_temp: ThermodynamicTemperature,
        left_downcomer_mid_temp: ThermodynamicTemperature,
        left_downcomer_lower_temp: ThermodynamicTemperature,
        right_downcomer_upper_temp: ThermodynamicTemperature,
        right_downcomer_mid_temp: ThermodynamicTemperature,
        right_downcomer_lower_temp: ThermodynamicTemperature,

        ) -> Self {
        Self { 
            size, 
            left_control_rod_insertion_frac: 1.0, 
            right_control_rod_insertion_frac: 1.0,
            min_temp,
            max_temp,
            pebble_core_temp,
            core_mid_temp: pebble_bed_coolant_temp,
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

    /// sets minimum temperature for colour
    pub fn set_min_temp(&mut self, min_temp: ThermodynamicTemperature,){
        self.min_temp = min_temp;
    }
    /// sets maximum temperature for colour
    pub fn set_max_temp(&mut self, max_temp: ThermodynamicTemperature,){
        self.max_temp = max_temp;
    }

    /// gets the size of the widget 
    pub fn size(&self) -> Vec2 {

        self.size.clone()
    }

    // sets the left control rod insertion fraction
    pub fn set_left_cr_frac(
        &mut self, 
        left_control_rod_insertion_frac: f32){
        self.left_control_rod_insertion_frac = left_control_rod_insertion_frac;
    }
    // sets the right control rod insertion fraction
    pub fn set_right_cr_frac(
        &mut self, 
        right_control_rod_insertion_frac: f32){
        self.right_control_rod_insertion_frac = right_control_rod_insertion_frac;
    }
}

impl Widget for FHRReactorWidget {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {

        let size = self.size();
        let (response, painter) = ui.allocate_painter(
            size, Sense::hover()
        );


        if self.left_control_rod_insertion_frac > 1.0 {
            self.left_control_rod_insertion_frac = 1.0;
        } else if self.left_control_rod_insertion_frac < 0.0 {
            self.left_control_rod_insertion_frac = 0.0;
        };
        if self.right_control_rod_insertion_frac > 1.0 {
            self.right_control_rod_insertion_frac = 1.0;
        } else if self.right_control_rod_insertion_frac < 0.0 {
            self.right_control_rod_insertion_frac = 0.0;
        };

        let rect = response.rect;
        let c = rect.center();

        let rect_x = rect.width();
        let rect_y = rect.height();

        let reactor_half_width_x = rect_x * 0.5;
        let reactor_half_length_y = rect_y * 0.25;
        // bottom inlet
        let fhr_coolant_inlet_bottom_left = c + vec2(-0.10*reactor_half_width_x, reactor_half_length_y* 0.9);
        let fhr_coolant_inlet_bottom_right = c + vec2(0.10*reactor_half_width_x, reactor_half_length_y* 0.9);

        // core part
        let fhr_core_inlet_bottom_left = c + vec2(-0.10*reactor_half_width_x, reactor_half_length_y* 0.65);
        let fhr_core_inlet_bottom_right = c + vec2(0.10*reactor_half_width_x, reactor_half_length_y* 0.65);
        let fhr_core_fat_bottom_left = c + vec2(-0.50*reactor_half_width_x, reactor_half_length_y* 0.45);
        let fhr_core_fat_bottom_right = c + vec2(0.50*reactor_half_width_x, reactor_half_length_y* 0.45);

        let fhr_core_fat_top_left = c + vec2(-0.50*reactor_half_width_x, -reactor_half_length_y* 0.45);
        let fhr_core_fat_top_right = c + vec2(0.50*reactor_half_width_x, -reactor_half_length_y* 0.45);
        let fhr_core_outlet_top_left = c + vec2(-0.10*reactor_half_width_x, -reactor_half_length_y* 0.65);
        let fhr_core_outlet_top_right = c + vec2(0.10*reactor_half_width_x, -reactor_half_length_y* 0.65);

        // top outlet
        let fhr_coolant_outlet_top_left = c + vec2(-0.10*reactor_half_width_x, -reactor_half_length_y* 0.9);
        let fhr_coolant_outlet_top_right = c + vec2(0.10*reactor_half_width_x, -reactor_half_length_y* 0.9);

        // colour fill
        let hotness: f32 = 0.1;
        let coolant_fill = hot_to_cold_colour_mark_1(hotness);

        // draw clockwise
        let core_bottom_points = vec![
            fhr_core_inlet_bottom_left,
            fhr_core_fat_bottom_left,
            fhr_core_fat_bottom_right,
            fhr_core_inlet_bottom_right,
        ];
        let core_bottom_inlet_points = vec![
            fhr_coolant_inlet_bottom_left,
            fhr_core_inlet_bottom_left,
            fhr_core_inlet_bottom_right,
            fhr_coolant_inlet_bottom_right,
        ];
        let core_mid_points = vec![
            fhr_core_fat_bottom_left,
            fhr_core_fat_top_left,
            fhr_core_fat_top_right,
            fhr_core_fat_bottom_right,
        ];
        let core_top_points = vec![
            fhr_core_fat_top_left,
            fhr_core_outlet_top_left,
            fhr_core_outlet_top_right,
            fhr_core_fat_top_right,
        ];
        let core_outlet_points = vec![
            //fhr_coolant_inlet_bottom_left,
            //fhr_core_inlet_bottom_left,
            //fhr_core_fat_bottom_left,
            //fhr_core_fat_top_left,
            fhr_core_outlet_top_left,
            fhr_coolant_outlet_top_left,
            fhr_coolant_outlet_top_right,
            fhr_core_outlet_top_right,
            //fhr_core_fat_top_right,
            //fhr_core_fat_bottom_right,
            //fhr_core_inlet_bottom_right,
            //fhr_coolant_inlet_bottom_right,
        ];

        // fhr metal container (grey colour)
        //
        // we will use a cubic Beizier curve


        let reactor_box_top_left = 
            c + vec2(-reactor_half_width_x, -reactor_half_length_y);
        let reactor_box_bottom_left = 
            c + vec2(-reactor_half_width_x, reactor_half_length_y);
        let reactor_box_top_right = 
            c + vec2(reactor_half_width_x, -reactor_half_length_y);
        let reactor_box_bottom_right = 
            c + vec2(reactor_half_width_x, reactor_half_length_y);


        let reactor_curved_edge_fraction = 0.55;

        let reactor_curved_edge_top_left = 
            c + vec2(-reactor_half_width_x, -reactor_curved_edge_fraction * reactor_half_length_y);
        let reactor_curved_edge_bottom_left = 
            c + vec2(-reactor_half_width_x, reactor_curved_edge_fraction * reactor_half_length_y);
        let reactor_curved_edge_top_right = 
            c + vec2(reactor_half_width_x, -reactor_curved_edge_fraction * reactor_half_length_y);
        let reactor_curved_edge_bottom_right = 
            c + vec2(reactor_half_width_x, reactor_curved_edge_fraction * reactor_half_length_y);

        let metal_fill = Color32::GRAY;

        let fhr_bottom_metal_pts = 
            [
            reactor_curved_edge_bottom_left,
            reactor_box_bottom_left,
            reactor_box_bottom_right,
            reactor_curved_edge_bottom_right
            ];
        let fhr_top_metal_pts = 
            [
            reactor_curved_edge_top_left,
            reactor_box_top_left,
            reactor_box_top_right,
            reactor_curved_edge_top_right
            ];

        let fhr_mid_metal_pts = 
            [
            reactor_curved_edge_bottom_left,
            reactor_curved_edge_top_left,
            reactor_curved_edge_top_right,
            reactor_curved_edge_bottom_right
            ];

        let color = Color32::from_gray(128);
        let stroke = Stroke::new(1.0, color);
        let fhr_bottom_metal_semicircle = 
            CubicBezierShape::from_points_stroke(fhr_bottom_metal_pts, 
                true, 
                metal_fill, 
                stroke);

        let fhr_top_metal_semicircle = 
            CubicBezierShape::from_points_stroke(fhr_top_metal_pts, 
                true, 
                metal_fill, 
                stroke);
        let fhr_mid_metal_rect = 
            PathShape::convex_polygon(fhr_mid_metal_pts.into(), 
                metal_fill, 
                stroke);

        // fhr metal vessel
        painter.add(fhr_bottom_metal_semicircle);
        painter.add(fhr_top_metal_semicircle);
        painter.add(fhr_mid_metal_rect);
        // inner graphite reflector 

        let graphite_width_fraction = 0.8;

        let reflector_box_top_left = 
            c + vec2(-reactor_half_width_x * graphite_width_fraction, -reactor_half_length_y * graphite_width_fraction);
        let reflector_box_bottom_left = 
            c + vec2(-reactor_half_width_x * graphite_width_fraction, reactor_half_length_y * graphite_width_fraction);
        let reflector_box_top_right = 
            c + vec2(reactor_half_width_x * graphite_width_fraction, -reactor_half_length_y * graphite_width_fraction);
        let reflector_box_bottom_right = 
            c + vec2(reactor_half_width_x * graphite_width_fraction, reactor_half_length_y * graphite_width_fraction);


        let reflector_curved_edge_fraction = 0.55;

        let reflector_curved_edge_top_left = 
            c + vec2(-reactor_half_width_x * graphite_width_fraction, -reflector_curved_edge_fraction * reactor_half_length_y);
        let reflector_curved_edge_bottom_left = 
            c + vec2(-reactor_half_width_x * graphite_width_fraction, reflector_curved_edge_fraction * reactor_half_length_y);
        let reflector_curved_edge_top_right = 
            c + vec2(reactor_half_width_x * graphite_width_fraction, -reflector_curved_edge_fraction * reactor_half_length_y);
        let reflector_curved_edge_bottom_right = 
            c + vec2(reactor_half_width_x * graphite_width_fraction, reflector_curved_edge_fraction * reactor_half_length_y);


        let reflector_bottom_graphite_pts = 
            [
            reflector_curved_edge_bottom_left,
            reflector_box_bottom_left,
            reflector_box_bottom_right,
            reflector_curved_edge_bottom_right
            ];
        let reflector_top_graphite_pts = 
            [
            reflector_curved_edge_top_left,
            reflector_box_top_left,
            reflector_box_top_right,
            reflector_curved_edge_top_right
            ];

        let reflector_mid_graphite_pts = 
            [
            reflector_curved_edge_bottom_left,
            reflector_curved_edge_top_left,
            reflector_curved_edge_top_right,
            reflector_curved_edge_bottom_right
            ];
        let graphite_fill = Color32::BLACK;

        let graphite_stroke = Stroke::new(1.0, graphite_fill);
        let reflector_bottom_graphite_semicircle = 
            CubicBezierShape::from_points_stroke(reflector_bottom_graphite_pts, 
                true, 
                graphite_fill, 
                graphite_stroke);

        let reflector_top_graphite_semicircle = 
            CubicBezierShape::from_points_stroke(reflector_top_graphite_pts, 
                true, 
                graphite_fill, 
                graphite_stroke);
        let reflector_mid_graphite_rect = 
            PathShape::convex_polygon(reflector_mid_graphite_pts.into(), 
                graphite_fill, 
                graphite_stroke);

        // fhr reflector graphite
        painter.add(reflector_bottom_graphite_semicircle);
        painter.add(reflector_top_graphite_semicircle);
        painter.add(reflector_mid_graphite_rect);

        
        let coolant_stroke = Stroke::new(1.0, coolant_fill);
        // fhr coolant 
        let core_bottom_hotness = 
            self.hotness(self.core_bottom_temp);
        let core_bottom_colour = hot_to_cold_colour_mark_1(
            core_bottom_hotness
        );
        let fhr_core_bottom_coolant_shape = 
            PathShape::convex_polygon(
                core_bottom_points, 
                core_bottom_colour, 
                coolant_stroke);

        let core_inlet_hotness = 
            self.hotness(self.core_inlet_temp);
        let core_inlet_colour = hot_to_cold_colour_mark_1(
            core_inlet_hotness
        );
        let fhr_core_inlet_coolant_shape = 
            PathShape::convex_polygon(
                core_bottom_inlet_points, 
                core_inlet_colour, 
                coolant_stroke);
        let core_mid_hotness = 
            self.hotness(self.core_mid_temp);
        let core_mid_colour = hot_to_cold_colour_mark_1(
            core_mid_hotness
        );
        let fhr_core_mid_coolant_shape = 
            PathShape::convex_polygon(
                core_mid_points, 
                core_mid_colour, 
                stroke);
        let fhr_core_top_coolant_shape = 
            PathShape::convex_polygon(
                core_top_points, 
                coolant_fill, 
                coolant_stroke);
        let fhr_core_outlet_coolant_shape = 
            PathShape::convex_polygon(
                core_outlet_points, 
                coolant_fill, 
                coolant_stroke);



        painter.add(fhr_core_bottom_coolant_shape);
        painter.add(fhr_core_inlet_coolant_shape);
        painter.add(fhr_core_top_coolant_shape);
        painter.add(fhr_core_outlet_coolant_shape);
        painter.add(fhr_core_mid_coolant_shape);

        // now for pebble bed 
        //
        let fhr_width = reactor_half_width_x * 2.0;
        let fhr_height = reactor_half_length_y * 2.0;
        let pebble_radius = fhr_width * 0.042;
        let core_radius = pebble_radius * 0.8;
        let pebble_ctr = c;

        painter.circle_filled(pebble_ctr, pebble_radius, Color32::BLACK);
        painter.circle_filled(pebble_ctr, core_radius, Color32::DARK_RED);

        let pebble_centers = vec![
            c + vec2(2.0*pebble_radius,0.1*pebble_radius),
            c + vec2(1.0*pebble_radius,-0.5*pebble_radius),
            c + vec2(4.0*pebble_radius,-0.3*pebble_radius),
            c + vec2(-2.0*pebble_radius,0.1*pebble_radius),
            c + vec2(-1.0*pebble_radius,-0.5*pebble_radius),
            c + vec2(-4.0*pebble_radius,-0.3*pebble_radius),
            c + vec2(2.0*pebble_radius,1.1*pebble_radius),
            c + vec2(1.0*pebble_radius,-1.5*pebble_radius),
            c + vec2(4.0*pebble_radius,-2.3*pebble_radius),
            c + vec2(-2.0*pebble_radius,1.1*pebble_radius),
            c + vec2(-1.0*pebble_radius,-2.5*pebble_radius),
            c + vec2(-4.0*pebble_radius,-2.3*pebble_radius),
            c + vec2(2.0*pebble_radius,1.1*pebble_radius),
            c + vec2(3.0*pebble_radius,-1.5*pebble_radius),
            c + vec2(5.0*pebble_radius,-2.3*pebble_radius),
            c + vec2(-2.0*pebble_radius,1.1*pebble_radius),
            c + vec2(-3.0*pebble_radius,-2.5*pebble_radius),
            c + vec2(-5.0*pebble_radius,-2.3*pebble_radius),
            c + vec2(-5.0*pebble_radius,2.3*pebble_radius),
            c + vec2(5.0*pebble_radius,2.3*pebble_radius),
            c + vec2(-4.2*pebble_radius,1.3*pebble_radius),
            c + vec2(4.0*pebble_radius,1.4*pebble_radius),
            c + vec2(-0.2*pebble_radius,1.3*pebble_radius),
            c + vec2(0.0*pebble_radius,1.8*pebble_radius),
            ];

        // add another list but transpose upwards 
        let mut pebble_centres_bottom: Vec<Pos2> = pebble_centers.clone();
        let mut pebble_centres_top: Vec<Pos2> = pebble_centers.clone();

        for (i,pebble_center) in pebble_centers.iter().enumerate() {
            pebble_centres_bottom[i] = 
                *pebble_center + vec2(0.0, fhr_height * 0.1);
        }
        for (i,pebble_center) in pebble_centers.iter().enumerate() {
            pebble_centres_top[i] = 
                *pebble_center + vec2(0.0, -fhr_height * 0.1);
        }


        for pebble_center in pebble_centers.iter(){
            painter.circle_filled(*pebble_center, pebble_radius, Color32::BLACK);
            painter.circle_filled(*pebble_center, core_radius, Color32::DARK_RED);
        }

        for pebble_center in pebble_centres_bottom.iter(){
            painter.circle_filled(*pebble_center, pebble_radius, Color32::BLACK);
            painter.circle_filled(*pebble_center, core_radius, Color32::DARK_RED);
        }
        for pebble_center in pebble_centres_top.iter(){
            painter.circle_filled(*pebble_center, pebble_radius, Color32::BLACK);
            painter.circle_filled(*pebble_center, core_radius, Color32::DARK_RED);
        }

        // next, downcomers

        // left downcomer inlet
        let left_downcomer_inlet_bottom_pt = 
            fhr_coolant_inlet_bottom_left 
            + vec2(0.0, -reactor_half_length_y * 0.04);

        let left_downcomer_inlet_top_pt = 
            fhr_coolant_inlet_bottom_left 
            + vec2(0.0, -reactor_half_length_y * 0.12);

        let left_downcomer_inlet_mid_bottom_pt = 
            fhr_coolant_inlet_bottom_left 
            + vec2(-reactor_half_width_x * 0.65, -reactor_half_length_y * 0.16);

        let left_downcomer_inlet_mid_top_pt = 
            fhr_coolant_inlet_bottom_left 
            + vec2(-reactor_half_width_x * 0.6, -reactor_half_length_y * 0.22);

        // left downcomer mid rectangle
        //
        let left_downcomer_mid_rect_bottom_left =
            reactor_curved_edge_bottom_left 
            + vec2(reactor_half_width_x *0.06, 0.0);

        let left_downcomer_mid_rect_bottom_right =
            reactor_curved_edge_bottom_left 
            + vec2(reactor_half_width_x *0.16, 0.0);


        let left_downcomer_mid_rect_top_left =
            reactor_curved_edge_top_left 
            + vec2(reactor_half_width_x *0.06, 0.0);

        let left_downcomer_mid_rect_top_right =
            reactor_curved_edge_top_left 
            + vec2(reactor_half_width_x *0.16, 0.0);

        // left downcomer outlet

        let left_downcomer_outlet_top_pt = 
            fhr_coolant_outlet_top_left 
            + vec2(0.0, reactor_half_length_y * 0.04);

        let left_downcomer_outlet_bottom_pt = 
            fhr_coolant_outlet_top_left 
            + vec2(0.0, reactor_half_length_y * 0.12);

        let left_downcomer_outlet_mid_bottom_pt = 
            fhr_coolant_outlet_top_left 
            + vec2(-reactor_half_width_x * 0.6, reactor_half_length_y * 0.22);

        let left_downcomer_outlet_mid_top_pt = 
            fhr_coolant_outlet_top_left 
            + vec2(-reactor_half_width_x * 0.65, reactor_half_length_y * 0.16);



        let downcomer_inlet_left_1_pts = 
            vec![
            left_downcomer_inlet_bottom_pt,
            left_downcomer_inlet_mid_bottom_pt,
            left_downcomer_inlet_mid_top_pt,
            left_downcomer_inlet_top_pt
            ];

        let downcomer_inlet_left_2_pts = 
            vec![
            left_downcomer_mid_rect_bottom_left,
            left_downcomer_mid_rect_bottom_right,
            left_downcomer_inlet_mid_top_pt,
            left_downcomer_inlet_mid_bottom_pt,
            ];


        let downcomer_left_mid_pts = 
            vec![
            left_downcomer_mid_rect_bottom_left,
            left_downcomer_mid_rect_top_left,
            left_downcomer_mid_rect_top_right,
            left_downcomer_mid_rect_bottom_right,
            ];

        let downcomer_outlet_left_1_pts = 
            vec![
            left_downcomer_outlet_bottom_pt,
            left_downcomer_outlet_mid_bottom_pt,
            left_downcomer_outlet_mid_top_pt,
            left_downcomer_outlet_top_pt
            ];

        let downcomer_outlet_left_2_pts = 
            vec![
            left_downcomer_mid_rect_top_left,
            left_downcomer_outlet_mid_top_pt,
            left_downcomer_outlet_mid_bottom_pt,
            left_downcomer_mid_rect_top_right,
            ];

        let left_downcomer_inlet_1_shape = 
            PathShape::convex_polygon(
                downcomer_inlet_left_1_pts, 
                coolant_fill, 
                coolant_stroke);
        let left_downcomer_inlet_2_shape = 
            PathShape::convex_polygon(
                downcomer_inlet_left_2_pts, 
                coolant_fill, 
                coolant_stroke);
        let left_downcomer_mid_shape = 
            PathShape::convex_polygon(
                downcomer_left_mid_pts, 
                coolant_fill, 
                coolant_stroke);
        let left_downcomer_outlet_1_shape = 
            PathShape::convex_polygon(
                downcomer_outlet_left_1_pts, 
                coolant_fill, 
                coolant_stroke);
        let left_downcomer_outlet_2_shape = 
            PathShape::convex_polygon(
                downcomer_outlet_left_2_pts, 
                coolant_fill, 
                coolant_stroke);

        painter.add(left_downcomer_inlet_1_shape);
        painter.add(left_downcomer_inlet_2_shape);
        painter.add(left_downcomer_mid_shape);
        painter.add(left_downcomer_outlet_1_shape);
        painter.add(left_downcomer_outlet_2_shape);

        // right downcomer 

        // right downcomer inlet
        let right_downcomer_inlet_bottom_pt = 
            fhr_coolant_inlet_bottom_right 
            + vec2(0.0, -reactor_half_length_y * 0.04);

        let right_downcomer_inlet_top_pt = 
            fhr_coolant_inlet_bottom_right 
            + vec2(0.0, -reactor_half_length_y * 0.12);

        let right_downcomer_inlet_mid_bottom_pt = 
            fhr_coolant_inlet_bottom_right 
            + vec2(reactor_half_width_x * 0.65, -reactor_half_length_y * 0.16);

        let right_downcomer_inlet_mid_top_pt = 
            fhr_coolant_inlet_bottom_right 
            + vec2(reactor_half_width_x * 0.6, -reactor_half_length_y * 0.22);

        // right downcomer mid rectangle
        //
        let right_downcomer_mid_rect_bottom_left =
            reactor_curved_edge_bottom_right 
            + vec2(-reactor_half_width_x *0.16, 0.0);

        let right_downcomer_mid_rect_bottom_right =
            reactor_curved_edge_bottom_right 
            + vec2(-reactor_half_width_x *0.06, 0.0);


        let right_downcomer_mid_rect_top_left =
            reactor_curved_edge_top_right 
            + vec2(-reactor_half_width_x *0.16, 0.0);

        let right_downcomer_mid_rect_top_right =
            reactor_curved_edge_top_right 
            + vec2(-reactor_half_width_x *0.06, 0.0);

        // right downcomer outlet

        let right_downcomer_outlet_top_pt = 
            fhr_coolant_outlet_top_right 
            + vec2(0.0, reactor_half_length_y * 0.04);

        let right_downcomer_outlet_bottom_pt = 
            fhr_coolant_outlet_top_right 
            + vec2(0.0, reactor_half_length_y * 0.12);

        let right_downcomer_outlet_mid_bottom_pt = 
            fhr_coolant_outlet_top_right 
            + vec2(reactor_half_width_x * 0.6, reactor_half_length_y * 0.22);

        let right_downcomer_outlet_mid_top_pt = 
            fhr_coolant_outlet_top_right 
            + vec2(reactor_half_width_x * 0.65, reactor_half_length_y * 0.16);

        let downcomer_inlet_right_1_pts = 
            vec![
            right_downcomer_inlet_bottom_pt,
            right_downcomer_inlet_mid_bottom_pt,
            right_downcomer_inlet_mid_top_pt,
            right_downcomer_inlet_top_pt
            ];

        let downcomer_inlet_right_2_pts = 
            vec![
            right_downcomer_inlet_mid_top_pt,
            right_downcomer_mid_rect_bottom_left,
            right_downcomer_mid_rect_bottom_right,
            right_downcomer_inlet_mid_bottom_pt,
            ];


        let downcomer_right_mid_pts = 
            vec![
            right_downcomer_mid_rect_bottom_left,
            right_downcomer_mid_rect_top_left,
            right_downcomer_mid_rect_top_right,
            right_downcomer_mid_rect_bottom_right,
            ];

        let downcomer_outlet_right_1_pts = 
            vec![
            right_downcomer_outlet_bottom_pt,
            right_downcomer_outlet_mid_bottom_pt,
            right_downcomer_outlet_mid_top_pt,
            right_downcomer_outlet_top_pt
            ];

        let downcomer_outlet_right_2_pts = 
            vec![
            right_downcomer_outlet_mid_top_pt,
            right_downcomer_mid_rect_top_right,
            right_downcomer_mid_rect_top_left,
            right_downcomer_outlet_mid_bottom_pt,
            ];

        let right_downcomer_inlet_1_shape = 
            PathShape::convex_polygon(
                downcomer_inlet_right_1_pts, 
                coolant_fill, 
                coolant_stroke);
        let right_downcomer_inlet_2_shape = 
            PathShape::convex_polygon(
                downcomer_inlet_right_2_pts, 
                coolant_fill, 
                coolant_stroke);
        let right_downcomer_mid_shape = 
            PathShape::convex_polygon(
                downcomer_right_mid_pts, 
                coolant_fill, 
                coolant_stroke);
        let right_downcomer_outlet_1_shape = 
            PathShape::convex_polygon(
                downcomer_outlet_right_1_pts, 
                coolant_fill, 
                coolant_stroke);
        let right_downcomer_outlet_2_shape = 
            PathShape::convex_polygon(
                downcomer_outlet_right_2_pts, 
                coolant_fill, 
                coolant_stroke);

        painter.add(right_downcomer_inlet_1_shape);
        painter.add(right_downcomer_inlet_2_shape);
        painter.add(right_downcomer_mid_shape);
        painter.add(right_downcomer_outlet_1_shape);
        painter.add(right_downcomer_outlet_2_shape);

        // now control rods

        let cr_channel_length_ratio = 1.0;
        let cr_channel_width_ratio = 0.08;

        let cr_left_ref_pt = 
            reflector_curved_edge_top_left 
            + vec2(reactor_half_width_x * 0.15, 0.0);

        let left_cr_channel_top_left = 
            cr_left_ref_pt 
            + vec2(-reactor_half_width_x * cr_channel_width_ratio, 0.0);

        let left_cr_channel_top_right = 
            cr_left_ref_pt 
            + vec2(reactor_half_width_x * cr_channel_width_ratio, 0.0);

        let left_cr_channel_bottom_left = 
            cr_left_ref_pt 
            + vec2(-reactor_half_width_x * 0.08, 
                reactor_half_length_y * cr_channel_length_ratio);

        let left_cr_channel_bottom_right = 
            cr_left_ref_pt 
            + vec2(reactor_half_width_x * 0.08, 
                reactor_half_length_y * cr_channel_length_ratio);

        let cr_channel_fill = Color32::LIGHT_BLUE;




        let left_cr_channel_pts = 
            vec![
            left_cr_channel_top_left,
            left_cr_channel_top_right,
            left_cr_channel_bottom_right,
            left_cr_channel_bottom_left,
            ];

        let cr_left_channel_shape = 
            PathShape::convex_polygon(
                left_cr_channel_pts, 
                cr_channel_fill, 
                coolant_stroke);


        let cr_right_ref_pt = 
            reflector_curved_edge_top_right 
            + vec2(-reactor_half_width_x * 0.15, 0.0);

        let right_cr_channel_top_left = 
            cr_right_ref_pt 
            + vec2(-reactor_half_width_x * cr_channel_width_ratio, 0.0);

        let right_cr_channel_top_right = 
            cr_right_ref_pt 
            + vec2(reactor_half_width_x * cr_channel_width_ratio, 0.0);

        let right_cr_channel_bottom_left = 
            cr_right_ref_pt 
            + vec2(-reactor_half_width_x * 0.08, 
                reactor_half_length_y * cr_channel_length_ratio);

        let right_cr_channel_bottom_right = 
            cr_right_ref_pt 
            + vec2(reactor_half_width_x * 0.08, 
                reactor_half_length_y * cr_channel_length_ratio);
        let right_cr_channel_pts = 
            vec![
            right_cr_channel_top_left,
            right_cr_channel_top_right,
            right_cr_channel_bottom_right,
            right_cr_channel_bottom_left,
            ];

        let cr_right_channel_shape = 
            PathShape::convex_polygon(
                right_cr_channel_pts, 
                cr_channel_fill, 
                coolant_stroke);

        painter.add(cr_left_channel_shape);
        painter.add(cr_right_channel_shape);

        let cr_width_ratio = 0.08;
        let cr_colour = Color32::DARK_GRAY;


        let cr_rod_stroke = Stroke::new(
            cr_width_ratio * reactor_half_width_x, 
            cr_colour
        );

        let cr_length = reactor_half_length_y * 0.88;

        let cr_left_centre = cr_left_ref_pt 
            + vec2(0.0, cr_length*self.left_control_rod_insertion_frac - cr_length*0.9);

        let cr_right_centre = cr_right_ref_pt 
            + vec2(0.0, cr_length*self.right_control_rod_insertion_frac - cr_length*0.9);


        painter.line_segment(
            [cr_left_centre - vec2(0.0, cr_length), 
            cr_left_centre + vec2(0.0, cr_length)], 
            cr_rod_stroke
        );
        painter.line_segment(
            [cr_right_centre - vec2(0.0, cr_length), 
            cr_right_centre + vec2(0.0, cr_length)], 
            cr_rod_stroke
        );

        response

            //let (rect, mut response) = ui.allocate_at_least(desired_size, sense);

            //response.widget_info(|| {
            //    if let Some(galley) = &galley {
            //        WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), galley.text())
            //    } else {
            //        WidgetInfo::new(WidgetType::Button)
            //    }
            //});


            //if ui.is_rect_visible(rect) {
            //    let visuals = ui.style().interact(&response);
            //    let (frame_expansion, frame_cr, frame_fill, frame_stroke) = if selected {
            //        let selection = ui.visuals().selection;
        //        (
        //            Vec2::ZERO,
        //            CornerRadius::ZERO,
        //            selection.bg_fill,
        //            selection.stroke,
        //        )
        //    } else if frame {
        //        let expansion = Vec2::splat(visuals.expansion);
        //        (
        //            expansion,
        //            visuals.corner_radius,
        //            visuals.weak_bg_fill,
        //            visuals.bg_stroke,
        //        )
        //    } else {
        //        Default::default()
        //    };
        //
        //    let frame_cr = corner_radius.unwrap_or(frame_cr);
        //    let frame_fill = fill.unwrap_or(frame_fill);
        //    let frame_stroke = stroke.unwrap_or(frame_stroke);
        //    ui.painter().rect(
        //        rect.expand2(frame_expansion),
        //        frame_cr,
        //        frame_fill,
        //        frame_stroke,
        //        epaint::StrokeKind::Inside,
        //    );


        //    let mut cursor_x = rect.min.x + button_padding.x;
        //    if let Some(image) = &image {
        //        let mut image_pos = ui
        //            .layout()
        //            .align_size_within_rect(image_size, rect.shrink2(button_padding))
        //            .min;
        //        if galley.is_some() || shortcut_galley.is_some() {
        //            image_pos.x = cursor_x;
        //        }
        //        let image_rect = Rect::from_min_size(image_pos, image_size);
        //        cursor_x += image_size.x;
        //        let tlr = image.load_for_size(ui.ctx(), image_size);
        //        let mut image_options = image.image_options().clone();
        //        if image_tint_follows_text_color {
        //            image_options.tint = image_options.tint * visuals.text_color();
        //        }
        //        widgets::image::paint_texture_load_result(
        //            ui,
        //            &tlr,
        //            image_rect,
        //            image.show_loading_spinner,
        //            &image_options,
        //            None,
        //        );

        //        response = widgets::image::texture_load_result_response(
        //            &image.source(ui.ctx()),
        //            &tlr,
        //            response,
        //        );
        //    }
        //    if image.is_some() && galley.is_some() {
        //        cursor_x += ui.spacing().icon_spacing;
        //    }
        //    if let Some(galley) = galley {
        //        let mut text_pos = ui
        //            .layout()
        //            .align_size_within_rect(galley.size(), rect.shrink2(button_padding))
        //            .min;
        //        if image.is_some() || shortcut_galley.is_some() {
        //            text_pos.x = cursor_x;
        //        }
        //        ui.painter().galley(text_pos, galley, visuals.text_color());
        //    }


        //    if let Some(shortcut_galley) = shortcut_galley {
        //        // Always align to the right
        //        let layout = if ui.layout().is_horizontal() {
        //            ui.layout().with_main_align(Align::Max)
        //        } else {
        //            ui.layout().with_cross_align(Align::Max)
        //        };
        //        let shortcut_text_pos = layout
        //            .align_size_within_rect(shortcut_galley.size(), rect.shrink2(button_padding))
        //            .min;
        //        ui.painter().galley(
        //            shortcut_text_pos,
        //            shortcut_galley,
        //            ui.visuals().weak_text_color(),
        //        );
        //    }
        //}


        //if let Some(cursor) = ui.visuals().interact_cursor {
        //    if response.hovered() {
        //        ui.ctx().set_cursor_icon(cursor);
        //    }
        //}

        //response
    }
}
