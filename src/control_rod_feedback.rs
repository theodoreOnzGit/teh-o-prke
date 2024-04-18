use std::f64::consts::PI;

use uom::si::{f64::*, ratio::ratio};

use crate::teh_o_prke_error::TehOPrkeError;

/// based on Lamarsh's formula, obtain a rod worth for a cylinder 
/// of height H, and an insertion length of x
///
///
/// rho (x) = rho (H) * [x/H - 1/ (2 pi) sin (2 pi x/H)]
///
/// of course x is necessarily less than or equal H
pub fn obtain_rod_worth_cylinder(cylinder_height: Length,
    insertion_length: Length,
    rod_worth: Ratio) -> Result<Ratio, TehOPrkeError> {

    let mut x_by_h: Ratio = insertion_length/cylinder_height;

    // if insertion length is longer than the cylinder height,
    // then x_by_h is 1.0
    if x_by_h.get::<ratio>() > 1.0 {
        x_by_h = Ratio::new::<ratio>(1.0);
    }

    // [x/H - 1/ (2 pi) sin (2 pi x/H)]
    let rod_worth_ratio: Ratio = 
        x_by_h 
        - Ratio::new::<ratio>(
            1.0/(2.0* PI) * (2.0*PI *x_by_h.get::<ratio>()).sin()
        );

    let reactivity = rod_worth_ratio * rod_worth;


    Ok(reactivity)
}
