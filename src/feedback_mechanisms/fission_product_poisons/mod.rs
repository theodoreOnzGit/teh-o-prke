use uom::{si::f64::*, ConstZero};

#[derive(Debug, Clone, Copy)]
pub struct Xenon135Poisoning {
    pub iodine_135_concentration: VolumetricNumberDensity,
    pub xenon_135_concentration: VolumetricNumberDensity,
}

impl Xenon135Poisoning {

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
