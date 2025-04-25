use std::sync::{Arc, Mutex};

use teh_o_prke::zero_power_prke::six_group::SixGroupPRKE;
use uom::si::f64::*;
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::{FHRSimulatorApp, FHRState};

impl FHRSimulatorApp {

    /// associated function for PRKE calculation 
    /// for continuous loop
    pub fn calculate_prke_loop(
        fhr_state: Arc<Mutex<FHRState>>){

        let prke_six_group :SixGroupPRKE;

    }
    /// associated function for PRKE calculation 
    /// for single timestep
    pub fn calculate_prke_for_one_timestep(
        fhr_state: Arc<Mutex<FHRState>>){

        // within each timestep, I need to obtain feedback
        // so basically for now, fuel temperature and control rod insertion 
        // based on that, calculate PRKE

        let fhr_state_clone = fhr_state.lock().unwrap().clone();

        let fuel_temp: ThermodynamicTemperature = 
            ThermodynamicTemperature::new::<degree_celsius>(
                fhr_state_clone.pebble_core_temp_degc
            );

        let left_cr_insertion_frac = 
            fhr_state_clone.left_cr_insertion_frac;
        let right_cr_insertion_frac = 
            fhr_state_clone.right_cr_insertion_frac;


        

    }
}
