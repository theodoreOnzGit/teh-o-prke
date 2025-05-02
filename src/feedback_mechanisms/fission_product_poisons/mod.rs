use uom::{si::{f64::*, ratio::ratio}, ConstZero};

#[derive(Debug, Clone, Copy)]
pub struct Xenon135Poisoning {
    pub iodine_135_concentration: VolumetricNumberDensity,
    pub xenon_135_concentration: VolumetricNumberDensity,
}

impl Xenon135Poisoning {

    /// table 7.5 
    /// returns number of atoms per thermal fission of a nuclide
    /// Lamarsh, J. R. (1975). Introduction to nuclear engineering.
    #[inline]
    pub fn fp_yield_iodine_135_from_u235_thermal_fission() -> Ratio {
        Ratio::new::<ratio>(0.0639)
    }
    /// returns number of atoms per thermal fission of a nuclide
    /// Lamarsh, J. R. (1975). Introduction to nuclear engineering.
    #[inline]
    pub fn fp_yield_xe_135_from_u235_thermal_fission() -> Ratio {
        Ratio::new::<ratio>(0.00237)
    }

    /// table 7.5 
    /// returns number of atoms per thermal fission of a nuclide
    /// Lamarsh, J. R. (1975). Introduction to nuclear engineering.
    #[inline]
    pub fn fp_yield_iodine_135_from_u233_thermal_fission() -> Ratio {
        Ratio::new::<ratio>(0.0475)
    }
    /// table 7.5 
    /// returns number of atoms per thermal fission of a nuclide
    /// Lamarsh, J. R. (1975). Introduction to nuclear engineering.
    #[inline]
    pub fn fp_yield_xe_135_from_u233_thermal_fission() -> Ratio {
        Ratio::new::<ratio>(0.00107)
    }


    /// table 7.5 
    /// Lamarsh, J. R. (1975). Introduction to nuclear engineering.
    /// returns number of atoms per thermal fission of a nuclide
    #[inline]
    pub fn fp_yield_iodine_135_from_pu239_thermal_fission() -> Ratio {
        Ratio::new::<ratio>(0.0604)
    }
    /// table 7.5 
    /// Lamarsh, J. R. (1975). Introduction to nuclear engineering.
    /// returns number of atoms per thermal fission of a nuclide
    #[inline]
    pub fn fp_yield_xe_135_from_pu239_thermal_fission() -> Ratio {
        Ratio::new::<ratio>(0.00105)
    }
}

impl Default for Xenon135Poisoning {
    /// returns a fresh core
    fn default() -> Self {
        return Self {
            iodine_135_concentration: VolumetricNumberDensity::ZERO, 
            xenon_135_concentration: VolumetricNumberDensity::ZERO, 
        }
    }
}
