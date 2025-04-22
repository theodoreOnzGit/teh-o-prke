use std::ops::DerefMut;

use egui::Ui;

use crate::GuiClient;
use egui_plot::{Legend, Line, Plot, PlotPoints};

pub mod first_order_transfer_fn;
pub mod second_order_transfer_fn;
pub mod opcua_panel;
pub mod decaying_sinusoid;

impl GuiClient {
    pub fn simple_panel_ui(&mut self, ui: &mut Ui) {

        ui.heading(" 3.6 Roentgen... Not great not terrible");

        ui.horizontal(|ui| {
            ui.label("Write something: ");
            ui.text_edit_singleline(&mut self.label);
        });

        let mut binding = self.rad_value_ptr.lock().unwrap();
        let rad_value_ptr_clone = binding.deref_mut();

        ui.add(egui::Slider::new(rad_value_ptr_clone, 0.0..=15000.0).
            text("Roentgen/hr"));
        if ui.button("Increment").clicked() {
            *rad_value_ptr_clone += 1.0;
        }
        ui.add(egui::Spinner::new());
        let mut my_plot = Plot::new("My Plot").legend(Legend::default());

        // sets the aspect for plot 
        my_plot = my_plot.width(800.0);
        my_plot = my_plot.view_aspect(16.0/9.0);
        my_plot = my_plot.data_aspect(2.5);
        my_plot = my_plot.auto_bounds_x();
        my_plot = my_plot.auto_bounds_y();

        // let's create a line in the plot
        let plot_pts: Vec<[f64;2]> = self.plot_points_ptr.lock().unwrap().deref_mut()
            .iter().map(|&values|{
                values}
            ).collect();

        let time_vec: Vec<f64> = plot_pts.iter().map(
            |tuple|{
                let [time,_] = *tuple;

                time
            }
        ).collect();

        let y_vec: Vec<f64> = plot_pts.iter().map(
            |tuple|{
                let [_,y] = *tuple;

                y
            }
        ).collect();

        let max_time = time_vec.clone().into_iter().fold(f64::NEG_INFINITY, f64::max);
        let max_y = y_vec.into_iter().fold(f64::NEG_INFINITY, f64::max);

        // include max x and y values 
        my_plot = my_plot.include_x(max_time);
        my_plot = my_plot.include_y(max_y);

        // axis labels 
        my_plot = my_plot.x_axis_label(
            "time (seconds), current time (seconds): ".to_owned() 
            + &max_time.to_string());

        // now truncate values that are too old
        // show only last minute 
        let time_window_seconds = 60.0;
        if max_time as f64 > time_window_seconds as f64 {
            // i want to delete time older than time_window_seconds
            let index_result = time_vec.clone().iter().position(
                |&time| {
                    // we check if the time is less than the oldest 
                    // allowable time 
                    let oldest_allowable_time = max_time - time_window_seconds;
                    time < oldest_allowable_time
                }
            );
            let _ = match index_result {
                Some(index) => {
                    self.plot_points_ptr.lock().unwrap().deref_mut().remove(index);
                },
                None => {
                    // do nothing 
                    ()
                },
            };

        }



        my_plot.show(ui, |plot_ui| {
            plot_ui.line(Line::new(PlotPoints::from(
                        plot_pts
            )).name("user input"));
        });
    }

    pub fn transfer_fn_input_output_panel_ui(&mut self, ui: &mut Ui) {

        ui.separator();
        ui.add(egui::Spinner::new());

        let mut binding = self.user_input.lock().unwrap();
        let user_input_value = binding.deref_mut();
        ui.add(egui::Slider::new(user_input_value, 0.0..=0.9).
            text("units TBD"));


        let mut my_plot = Plot::new("My Plot").legend(Legend::default());

        // sets the aspect for plot 
        my_plot = my_plot.width(800.0);
        my_plot = my_plot.view_aspect(16.0/9.0);
        my_plot = my_plot.data_aspect(2.5);
        my_plot = my_plot.auto_bounds_x();
        my_plot = my_plot.auto_bounds_y();

        // let's create a line in the plot
        let input_output_plot_pts: Vec<[f64;3]> = self.
            input_output_plots_ptr.lock().unwrap().deref_mut()
            .iter().map(|&values|{
                values}
            ).collect();

        let time_vec: Vec<f64> = input_output_plot_pts.iter().map(
            |tuple|{
                let [time,_,_] = *tuple;

                time
            }
        ).collect();

        let user_input_vec: Vec<f64> = input_output_plot_pts.iter().map(
            |tuple|{
                let [_,user_input,_] = *tuple;

                user_input
            }
        ).collect();

        let time_input_vec: Vec<[f64;2]> = input_output_plot_pts.iter().map(
            |tuple|{
                let [time,user_input,_] = *tuple;

                [time, user_input]
            }
        ).collect();

        let time_output_vec: Vec<[f64;2]> = input_output_plot_pts.iter().map(
            |tuple|{
                let [time,_,model_output] = *tuple;

                [time, model_output]
            }
        ).collect();

        let max_time = time_vec.clone().into_iter().fold(f64::NEG_INFINITY, f64::max);
        let max_user_input = user_input_vec.clone().into_iter().fold(f64::NEG_INFINITY, f64::max);

        // include max x and y values 
        my_plot = my_plot.include_x(max_time);
        my_plot = my_plot.include_y(max_user_input);

        // axis labels 
        my_plot = my_plot.x_axis_label(
            "time (seconds), current time (seconds): ".to_owned() 
            + &max_time.to_string());

        // now truncate values that are too old
        // show only last minute 
        let time_window_seconds = 10.0;
        if max_time as f64 > time_window_seconds as f64 {
            // i want to delete time older than time_window_seconds
            let index_result = time_vec.clone().iter().position(
                |&time| {
                    // we check if the time is less than the oldest 
                    // allowable time 
                    let oldest_allowable_time = max_time - time_window_seconds;
                    time < oldest_allowable_time
                }
            );



            let _ = match index_result {
                Some(index) => {
                    self.input_output_plots_ptr.lock().unwrap().deref_mut().remove(index);
                },
                None => {
                    // do nothing 
                    ()
                },
            };

        }



        my_plot.show(ui, |plot_ui| {
            plot_ui.line(Line::new(PlotPoints::from(
                        time_input_vec
            )).name("user input"));
            plot_ui.line(Line::new(PlotPoints::from(
                        time_output_vec
            )).name("model input"));
        });
    }
}

