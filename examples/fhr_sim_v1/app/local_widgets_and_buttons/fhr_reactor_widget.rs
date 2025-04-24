use egui::{Rect, Sense, Vec2, Widget};

pub struct FHRReactorWidget {
    rect: Rect,
    left_control_rod_insertion_frac: f32,
    right_control_rod_insertion_frac: f32,
}

impl FHRReactorWidget {
    pub fn size(&self) -> Vec2 {

        let left_most_side = self.rect.left();
        let top_most_side = self.rect.top();
        let right_most_side = self.rect.right();
        let bottom_most_side = self.rect.bottom();

        let max_height_y = (top_most_side - bottom_most_side).abs();
        let max_width_x = (left_most_side - right_most_side).abs();


        // the size here is the size of the painter
        let size = Vec2::new(max_width_x, max_height_y);

        size
    }
}

impl Widget for FHRReactorWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {

        let size = self.size();
        let (response, painter) = ui.allocate_painter(
            size, Sense::hover()
        );

        let response_rect = response.rect;


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
