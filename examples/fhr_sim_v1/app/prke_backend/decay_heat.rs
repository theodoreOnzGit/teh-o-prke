use std::f64::consts::LN_2;
use uom::si::time::{day, hour};
use uom::si::{f64::*, ratio::ratio};
use uom::ConstZero;
/// this struct helps to manage decay heat calculations
///
/// similar to how delayed neutron precursors are handled,
/// I have some groups for decay heat precursors. 
/// These are loosely coupled (semi-implicit)
///
///
/// i think this is slightly buggy, need to change code
///
/// maybe the units and equations are wrong...
#[derive(Clone, Debug, Copy)]
pub struct FHRDecayHeat {
    decay_heat_precursor1: Power,
    pub decay_heat_precursor1_half_life: Time,
    decay_heat_precursor2: Power,
    pub decay_heat_precursor2_half_life: Time,
    decay_heat_precursor3: Power,
    pub decay_heat_precursor3_half_life: Time,
}

impl FHRDecayHeat {
    pub fn add_decay_heat_precursor1(&mut self, 
        decay_precursor_power: Power){

        self.decay_heat_precursor1 += decay_precursor_power;
    }
    pub fn add_decay_heat_precursor2(&mut self, 
        decay_precursor_power: Power){

        self.decay_heat_precursor2 += decay_precursor_power;
    }
    pub fn add_decay_heat_precursor3(&mut self, 
        decay_precursor_power: Power){

        self.decay_heat_precursor3 += decay_precursor_power;
    }


    /// basically 
    ///
    /// (P_decay^(t + Delta t) - P_decay^t)/ Delta t = - lambda_i P_decay^(t + Delta t)
    pub fn calc_decay_heat_power_1(&mut self,
        timestep: Time) -> Power {

        let decay_constant: Frequency = 
            LN_2/self.decay_heat_precursor1_half_life;
        // (P_decay^(t + Delta t) - P_decay^t)/ Delta t = - lambda_i P_decay^(t + Delta t)
        //
        // P_decay^(t + Delta t) (1 + lambda_i * Delta t) = P_decay^t
        //

        let p_decay_t = self.decay_heat_precursor1;
        let coeff = Ratio::new::<ratio>(1.0) + decay_constant * timestep;

        let p_decay_t_plus_delta_t = p_decay_t / coeff;

        self.decay_heat_precursor1 = p_decay_t_plus_delta_t;

        return p_decay_t_plus_delta_t;

        

    }
    /// basically 
    ///
    /// (P_decay^(t + Delta t) - P_decay^t)/ Delta t = - lambda_i P_decay^(t + Delta t)
    pub fn calc_decay_heat_power_2(&mut self,
        timestep: Time) -> Power {

        let decay_constant: Frequency = 
            LN_2/self.decay_heat_precursor2_half_life;
        // (P_decay^(t + Delta t) - P_decay^t)/ Delta t = - lambda_i P_decay^(t + Delta t)
        //
        // P_decay^(t + Delta t) (1 + lambda_i * Delta t) = P_decay^t
        //

        let p_decay_t = self.decay_heat_precursor2;
        let coeff = Ratio::new::<ratio>(1.0) + decay_constant * timestep;

        let p_decay_t_plus_delta_t = p_decay_t / coeff;

        self.decay_heat_precursor2 = p_decay_t_plus_delta_t;

        return p_decay_t_plus_delta_t;

        

    }
    /// basically 
    ///
    /// (P_decay^(t + Delta t) - P_decay^t)/ Delta t = - lambda_i P_decay^(t + Delta t)
    pub fn calc_decay_heat_power_3(&mut self,
        timestep: Time) -> Power {

        let decay_constant: Frequency = 
            LN_2/self.decay_heat_precursor3_half_life;
        // (P_decay^(t + Delta t) - P_decay^t)/ Delta t = - lambda_i P_decay^(t + Delta t)
        //
        // P_decay^(t + Delta t) (1 + lambda_i * Delta t) = P_decay^t
        //

        let p_decay_t = self.decay_heat_precursor3;
        let coeff = Ratio::new::<ratio>(1.0) + decay_constant * timestep;

        let p_decay_t_plus_delta_t = p_decay_t / coeff;

        self.decay_heat_precursor3 = p_decay_t_plus_delta_t;

        return p_decay_t_plus_delta_t;

        

    }

}

/// default is half lives of 
///
/// 0.2 hrs
/// 8 hrs 
/// 30 days 
///
/// then fission power ratio is:
/// 30 days -> 2%
/// 8 hrs -> 4% 
/// 0.2 hrs -> 4%
impl Default for FHRDecayHeat {
    fn default() -> Self {

        Self { 
            decay_heat_precursor1: Power::ZERO, 
            decay_heat_precursor1_half_life: Time::new::<hour>(0.2), 
            decay_heat_precursor2: Power::ZERO, 
            decay_heat_precursor2_half_life: Time::new::<hour>(8.0), 
            decay_heat_precursor3: Power::ZERO, 
            decay_heat_precursor3_half_life: Time::new::<day>(30.0),
        }
    }
}

