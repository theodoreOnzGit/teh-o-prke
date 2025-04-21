use core::f64;

use uom::si::{f64::*, ratio::ratio};

/// six factor formula to calculate keff and 
/// reactivity
///
/// keff = P_TNL * P_FNL * eta * f * p * epsilon
///
/// epsilon = fast fission factor 
/// p = resonance escape probability 
/// f  = thermal utilisation factor 
/// eta = fuel reproduction factor 
///
/// P_TNL = probability of thermal non leakage 
/// P_FNL = probability of fast non leakage
#[derive(Debug,Clone,Copy)]
pub struct SixFactorFormulaFeedback{

    /// thermal non leakage probability
    pub p_tnl: Ratio,

    /// fast non leakage probability
    pub p_fnl: Ratio,

    /// fast fission factor 
    pub epsilon: Ratio,

    /// resonance escape probability 
    pub p: Ratio,

    /// thermal utilisation factor
    pub f: Ratio,

    /// fuel reproduction factor 
    pub eta: Ratio,

}

impl SixFactorFormulaFeedback {
    /// calculates the k_eff given the 
    /// stored data of the six factor formula
    #[inline]
    pub fn calc_keff(&self) -> Ratio {
        return 
            self.epsilon 
            * self.p_fnl 
            * self.p 
            * self.p_tnl
            * self.f
            * self.eta;
    }

    /// calculates reactivity given the six factor 
    /// formula 
    /// rho = (keff - 1)/keff
    #[inline]
    pub fn calc_rho(&self) -> Ratio {
        let keff = self.calc_keff();
        // what if keff = 0 tho? 
        // we would just get negative infinity 
        // I'll just give a large negative number
        // arbitrarily large

        if keff.get::<ratio>() <= 0.0 {

            return Ratio::new::<ratio>(-1.0e30);
        }


        return (keff-Ratio::new::<ratio>(1.0))/keff;
    }


    /// fuel temperature feedback, should impact 
    /// resonance escape probability
    /// there should be a set fuel temperature to  
    /// resonance esc probability map or function
    pub fn fuel_temp_feedback(&mut self,
        t: ThermodynamicTemperature,
        resonance_esc_feedback: fn(ThermodynamicTemperature) -> Ratio ) 
        {

            // this is the user defined resonance 
            // escape probability feedback due to fuel 
            // temperature
            let p = resonance_esc_feedback(t);
            self.p = p;

        }
}

impl Default for SixFactorFormulaFeedback {
    fn default() -> Self {
        let ratio_one = Ratio::new::<ratio>(1.0);
        Self { 
            p_tnl: ratio_one, 
            p_fnl: ratio_one, 
            epsilon: ratio_one, 
            p: ratio_one, 
            f: ratio_one, 
            eta: ratio_one 
        }
    }
}
