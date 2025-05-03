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
/// the precursors are energy units, not power...
#[derive(Clone, Debug, Copy)]
pub struct DecayHeat {
    decay_heat_precursor1: Energy,
    pub decay_heat_precursor1_half_life: Time,
    decay_heat_precursor2: Energy,
    pub decay_heat_precursor2_half_life: Time,
    decay_heat_precursor3: Energy,
    pub decay_heat_precursor3_half_life: Time,
}

impl DecayHeat {
    pub fn add_decay_heat_precursor1(&mut self, 
        decay_precursor_power: Power,
        timestep: Time){

        self.decay_heat_precursor1 += decay_precursor_power * timestep;
    }
    pub fn add_decay_heat_precursor2(&mut self, 
        decay_precursor_power: Power,
        timestep: Time){

        self.decay_heat_precursor2 += decay_precursor_power * timestep;
    }
    pub fn add_decay_heat_precursor3(&mut self, 
        decay_precursor_power: Power,
        timestep: Time){

        self.decay_heat_precursor3 += decay_precursor_power * timestep;
    }


    /// basically 
    ///
    /// (E_decay^(t + Delta t) - E_decay^t)/ Delta t = - lambda_i E_decay^(t + Delta t)
    pub fn calc_decay_heat_power_1(&mut self,
        timestep: Time) -> Power {

        let decay_constant: Frequency = 
            LN_2/self.decay_heat_precursor1_half_life;
        // (E_decay^(t + Delta t) - E_decay^t)/ Delta t = - lambda_i E_decay^(t + Delta t)
        //
        // E_decay^(t + Delta t) (1 + lambda_i * Delta t) = E_decay^t
        //

        let e_decay_t = self.decay_heat_precursor1;
        let coeff = Ratio::new::<ratio>(1.0) + decay_constant * timestep;

        let e_decay_t_plus_delta_t = e_decay_t / coeff;

        self.decay_heat_precursor1 = e_decay_t_plus_delta_t;

        return (e_decay_t_plus_delta_t - e_decay_t)/timestep;

        

    }
    /// basically 
    ///
    /// (E_decay^(t + Delta t) - E_decay^t)/ Delta t = - lambda_i E_decay^(t + Delta t)
    pub fn calc_decay_heat_power_2(&mut self,
        timestep: Time) -> Power {

        let decay_constant: Frequency = 
            LN_2/self.decay_heat_precursor2_half_life;
        // (E_decay^(t + Delta t) - E_decay^t)/ Delta t = - lambda_i E_decay^(t + Delta t)
        //
        // E_decay^(t + Delta t) (1 + lambda_i * Delta t) = E_decay^t
        //

        let e_decay_t = self.decay_heat_precursor2;
        let coeff = Ratio::new::<ratio>(1.0) + decay_constant * timestep;

        let e_decay_t_plus_delta_t = e_decay_t / coeff;

        self.decay_heat_precursor2 = e_decay_t_plus_delta_t;

        return (e_decay_t_plus_delta_t - e_decay_t)/timestep;

        

    }
    /// basically 
    ///
    /// (E_decay^(t + Delta t) - E_decay^t)/ Delta t = - lambda_i E_decay^(t + Delta t)
    pub fn calc_decay_heat_power_3(&mut self,
        timestep: Time) -> Power {

        let decay_constant: Frequency = 
            LN_2/self.decay_heat_precursor3_half_life;
        // (E_decay^(t + Delta t) - E_decay^t)/ Delta t = - lambda_i E_decay^(t + Delta t)
        //
        // E_decay^(t + Delta t) (1 + lambda_i * Delta t) = E_decay^t
        //

        let e_decay_t = self.decay_heat_precursor3;
        let coeff = Ratio::new::<ratio>(1.0) + decay_constant * timestep;

        let e_decay_t_plus_delta_t = e_decay_t / coeff;

        self.decay_heat_precursor3 = e_decay_t_plus_delta_t;

        return (e_decay_t_plus_delta_t - e_decay_t)/timestep;

        

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
impl Default for DecayHeat {
    fn default() -> Self {

        Self { 
            decay_heat_precursor1: Energy::ZERO, 
            decay_heat_precursor1_half_life: Time::new::<hour>(0.2), 
            decay_heat_precursor2: Energy::ZERO, 
            decay_heat_precursor2_half_life: Time::new::<hour>(8.0), 
            decay_heat_precursor3: Energy::ZERO, 
            decay_heat_precursor3_half_life: Time::new::<day>(30.0),
        }
    }
}


