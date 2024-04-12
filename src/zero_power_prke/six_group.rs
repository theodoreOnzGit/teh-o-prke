use std::f64::consts::LN_2;

use uom::{si::{f64::*, ratio::ratio, time::second}, ConstZero};
use approx::*;
use ndarray::*;

// Decay Constant is essentially the same units as frequency
pub type DecayConstant = Frequency;
/// SixGroupPRKE
pub struct SixGroupPRKE {
    /// contains an array for the various half lives 
    /// of the
    half_life_array: [DecayConstant;6]
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


/// produces a new decay constant array
pub fn new_decay_constant_array() -> [DecayConstant;6] {

    // from table 5.1 (not sure where from, duderstadt?)
    // need to double check
    let mut decay_constant_array: [DecayConstant;6] = 
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
    //
    // lambda (decay constant) = ln(2)/(half life)
    decay_constant_array[0] = LN_2/half_life_array[0];
    decay_constant_array[1] = LN_2/half_life_array[1];
    decay_constant_array[2] = LN_2/half_life_array[2];
    decay_constant_array[3] = LN_2/half_life_array[3];
    decay_constant_array[4] = LN_2/half_life_array[4];
    decay_constant_array[5] = LN_2/half_life_array[5];

    // done!
    decay_constant_array
        

}

/// produces a new delayed fraction for u233 
pub fn new_u233_delayed_neutron_fraction_array() -> [Ratio;6] {

    let delayed_neutron_array: [Ratio;6] = 
        [
        Ratio::new::<ratio>(0.00023),
        Ratio::new::<ratio>(0.00078),
        Ratio::new::<ratio>(0.00064),
        Ratio::new::<ratio>(0.00074),
        Ratio::new::<ratio>(0.00014),
        Ratio::new::<ratio>(0.00008),
        ];

    delayed_neutron_array
}

/// produces a new delayed fraction for u235 
pub fn new_u235_delayed_neutron_fraction_array() -> [Ratio;6] {

    let delayed_neutron_array: [Ratio;6] = 
        [
        Ratio::new::<ratio>(0.00021),
        Ratio::new::<ratio>(0.00142),
        Ratio::new::<ratio>(0.00128),
        Ratio::new::<ratio>(0.00257),
        Ratio::new::<ratio>(0.00075),
        Ratio::new::<ratio>(0.00027),
        ];

    delayed_neutron_array
}


/// produces a new delayed fraction for pu239 
pub fn new_pu239_delayed_neutron_fraction_array() -> [Ratio;6] {

    let delayed_neutron_array: [Ratio;6] = 
        [
        Ratio::new::<ratio>(0.00007),
        Ratio::new::<ratio>(0.00063),
        Ratio::new::<ratio>(0.00044),
        Ratio::new::<ratio>(0.00069),
        Ratio::new::<ratio>(0.00018),
        Ratio::new::<ratio>(0.00009),
        ];

    delayed_neutron_array
}

#[test]
pub fn test_pu239_total_delayed_frac(){

    let total_delayed_fraction_reference = Ratio::new::<ratio>(0.00210);
    
    let pu239_delayed_frac_array = new_pu239_delayed_neutron_fraction_array();

    let test_delayed_frac: Ratio = pu239_delayed_frac_array.into_iter().sum();

    assert_eq!(test_delayed_frac,total_delayed_fraction_reference)


}

#[test]
pub fn test_u233_total_delayed_frac(){

    let total_delayed_fraction_reference = Ratio::new::<ratio>(0.00261);
    
    let u233_delayed_frac_array = new_u233_delayed_neutron_fraction_array();

    let test_delayed_frac: Ratio = u233_delayed_frac_array.into_iter().sum();

    assert_eq!(test_delayed_frac,total_delayed_fraction_reference)


}

#[test]
pub fn test_u235_total_delayed_frac(){

    let total_delayed_fraction_reference = Ratio::new::<ratio>(0.00650);
    
    let u235_delayed_frac_array = new_u235_delayed_neutron_fraction_array();

    let test_delayed_frac: Ratio = u235_delayed_frac_array.into_iter().sum();

    assert_abs_diff_eq!(
        test_delayed_frac.get::<ratio>(), 
        total_delayed_fraction_reference.get::<ratio>(), 
        epsilon = f64::EPSILON);


}
