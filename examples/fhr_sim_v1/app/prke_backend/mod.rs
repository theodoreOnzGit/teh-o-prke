use std::sync::{Arc, Mutex};

use teh_o_prke::{feedback_mechanisms::SixFactorFormulaFeedback, zero_power_prke::six_group::SixGroupPRKE};
use uom::si::{f64::*, ratio::ratio};
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::{FHRSimulatorApp, FHRState};

impl FHRSimulatorApp {

    /// associated function for PRKE calculation 
    /// for continuous loop
    pub fn calculate_prke_loop(
        fhr_state: Arc<Mutex<FHRState>>){

        // construct a prke six group object
        let mut prke_six_group :SixGroupPRKE = SixGroupPRKE::default();
        // now this is arbitrary, user can set
        let mut keff_six_factor = SixFactorFormulaFeedback::default();
        // start from keff = 1
        // all terms = 1
        // start with leakage
        keff_six_factor.p_tnl = Ratio::new::<ratio>(0.9);
        keff_six_factor.p_fnl = Ratio::new::<ratio>(0.7);
        // then fuel reproduction
        keff_six_factor.eta = Ratio::new::<ratio>(2.2);
        // resonance esc probability 
        keff_six_factor.p = Ratio::new::<ratio>(0.8);
        // thermal utilisation 
        keff_six_factor.f = Ratio::new::<ratio>(0.9);
        // fast fission 
        keff_six_factor.epsilon = Ratio::new::<ratio>(1.03);
        // keff total is about 1.0278
        // excess reactivity is about 0.0278
        //
        // basically control rod should be about this much 
        // and fuel temp feedback about this much also

        let mut fhr_state_clone = fhr_state.lock().unwrap().clone();
        loop {
            Self::calculate_prke_for_one_timestep(&mut fhr_state_clone);
        }

    }
    /// associated function for PRKE calculation 
    /// for single timestep
    pub fn calculate_prke_for_one_timestep(
        fhr_state_ref: &mut FHRState){

        // within each timestep, I need to obtain feedback
        // so basically for now, fuel temperature and control rod insertion 
        // based on that, calculate PRKE


        let fuel_temp: ThermodynamicTemperature = 
            ThermodynamicTemperature::new::<degree_celsius>(
                fhr_state_ref.pebble_core_temp_degc
            );

        let left_cr_insertion_frac = 
            fhr_state_ref.left_cr_insertion_frac;
        let right_cr_insertion_frac = 
            fhr_state_ref.right_cr_insertion_frac;

        // now based on this, calculate feedback


    }

    
    
}
