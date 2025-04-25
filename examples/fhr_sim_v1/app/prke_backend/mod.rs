use std::sync::{Arc, Mutex};

use crate::{FHRSimulatorApp, FHRState};

impl FHRSimulatorApp {

    /// associated function for PRKE calculation 
    /// for continuous loop
    pub fn calculate_prke_loop(
        fhr_state: Arc<Mutex<FHRState>>){


    }
    /// associated function for PRKE calculation 
    /// for single timestep
    pub fn calculate_prke_for_one_timestep(
        fhr_state: Arc<Mutex<FHRState>>){


    }
}
