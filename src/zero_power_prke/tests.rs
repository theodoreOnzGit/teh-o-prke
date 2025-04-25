use uom::si::{f64::*, ratio::ratio};

use crate::zero_power_prke::six_group::new_u235_delayed_neutron_fraction_array;
use crate::zero_power_prke::six_group::new_u233_delayed_neutron_fraction_array;
use crate::zero_power_prke::six_group::new_pu239_delayed_neutron_fraction_array;
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

    use approx::*;
    let total_delayed_fraction_reference = Ratio::new::<ratio>(0.00650);
    
    let u235_delayed_frac_array = new_u235_delayed_neutron_fraction_array();

    let test_delayed_frac: Ratio = u235_delayed_frac_array.into_iter().sum();

    assert_abs_diff_eq!(
        test_delayed_frac.get::<ratio>(), 
        total_delayed_fraction_reference.get::<ratio>(), 
        epsilon = f64::EPSILON);


}
