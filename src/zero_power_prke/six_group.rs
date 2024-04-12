use std::f64::consts::LN_2;

use uom::{si::{f64::*, time::second}, ConstZero};
use ndarray::*;

// Decay rate is essentially the same units as frequency
pub type DecayRate = Frequency;
/// SixGroupPRKE
pub struct SixGroupPRKE {
    /// contains an array for the various half lives 
    /// of the
    half_life_array: [DecayRate;6]
}

/// different nuclides or fuels have different delayed groups
pub enum DelayedGroupMode {
    /// chooses the U233 group of delayed constants
    U233,
    /// chooses the U235 group of delayed constants
    U235,
    /// chooses the Pu239 group of delayed constants
    Pu239
}


pub fn new_decay_constant_array() -> [DecayRate;6] {

    // from table 5.1 (not sure where from, duderstadt?)
    // need to double check
    let mut decay_constant_array: [DecayRate;6] = 
        [
        Frequency::ZERO,
        Frequency::ZERO,
        Frequency::ZERO,
        Frequency::ZERO,
        Frequency::ZERO,
        Frequency::ZERO,
        ];

    let half_life_array: [Time;6] = 
        [
        Time::new::<second>(56.0),
        Time::new::<second>(23.0),
        Time::new::<second>(6.2),
        Time::new::<second>(2.3),
        Time::new::<second>(0.61),
        Time::new::<second>(0.23),
        ];

    // crude, but gets the job done
    decay_constant_array[0] = LN_2/half_life_array[0];
    decay_constant_array[1] = LN_2/half_life_array[1];
    decay_constant_array[2] = LN_2/half_life_array[2];
    decay_constant_array[3] = LN_2/half_life_array[3];
    decay_constant_array[4] = LN_2/half_life_array[4];
    decay_constant_array[5] = LN_2/half_life_array[5];

    // done!
    decay_constant_array
        

}
