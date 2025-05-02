use uom::si::area::barn;
use uom::si::frequency::hertz;
use uom::si::velocity::meter_per_second;
use uom::ConstZero;
use uom::si::{f64::*, ratio::ratio};

use crate::zero_power_prke::six_group::FissioningNuclideType;

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

    /// table 7.6
    /// Lamarsh, J. R. (1975). Introduction to nuclear engineering.
    /// returns decay constant for a particular nuclide
    #[inline]
    pub fn iodine_135_decay_const() -> Frequency {
        Frequency::new::<hertz>(2.87e-5)
    }


    /// table 7.6
    /// Lamarsh, J. R. (1975). Introduction to nuclear engineering.
    /// returns decay constant for a particular nuclide
    #[inline]
    pub fn xe_135_decay_const() -> Frequency {
        Frequency::new::<hertz>(2.09e-5)
    }


    /// (dI/dt) = gamma_I * fission rate -  lambda_I * I
    ///
    /// (I ^(t + delta t) - I^t)/ (delta t) = gamma_I * fission_rate - lambda_I I^(t + delta t)
    /// (I ^(t + delta t) - I^t) = delta t * gamma_I * fission_rate - delta t * lambda_I I^(t + delta t)
    /// (I ^(t + delta t) + delta t * lambda_I I^(t + delta t)) = I^t + delta t * gamma_I * fission_rate 
    /// I ^(t + delta t)(1 + delta t * lambda_I ) = I^t + delta t * gamma_I * fission_rate 
    #[inline]
    fn calc_iodine_135_and_return_conc(&mut self,
        timestep: Time,
        fission_rate: VolumetricNumberRate,
        fissioning_nuclide: FissioningNuclideType) -> VolumetricNumberDensity {

        let gamma_i = match fissioning_nuclide {
            FissioningNuclideType::U233 => Self::fp_yield_iodine_135_from_u233_thermal_fission(),
            FissioningNuclideType::U235 => Self::fp_yield_iodine_135_from_u235_thermal_fission(),
            FissioningNuclideType::Pu239 => Self::fp_yield_iodine_135_from_pu239_thermal_fission(),
        };

        let current_iodine_conc = self.iodine_135_concentration;
        let additional_iodine_conc: VolumetricNumberDensity 
            = (timestep * fission_rate * gamma_i).into();

        let rhs = current_iodine_conc + additional_iodine_conc;

        let new_iodine_conc: VolumetricNumberDensity = 
            (
                rhs / (Ratio::new::<ratio>(1.0) + timestep * Self::iodine_135_decay_const())
            ).into();

        self.iodine_135_concentration = new_iodine_conc;

        return new_iodine_conc;

    }

    /// tentatively got from AI, but need to cite...
    #[inline]
    pub fn xe135_thermal_abs_xs() -> Area {
        Area::new::<barn>(2.65e6)
    }


    /// (dX/dt) = gamma_X * fission rate + lambda_I * I -  lambda_X * X - sigma_aX * X *
    /// thermal_flux
    ///
    /// upon discretisation (implicit)
    ///
    /// (X^(t + delta t) - X^t)/(delta t) = gamma_X * fission rate + lambda_I * I 
    /// -  lambda_X * X^(t + delta t) 
    /// - sigma_aX *  thermal_flux * X^(t + delta t)
    ///
    /// (X^(t + delta t) - X^t) = 
    /// delta t * gamma_X * fission rate 
    /// + delta t * lambda_I * I 
    /// -  delta t *lambda_X * X^(t + delta t)
    /// - delta t * sigma_aX *  thermal_flux * X^(t + delta t)
    ///
    ///
    /// (X^(t + delta t)) (1 +delta t *lambda_X + delta t * sigma_aX *  thermal_flux) = 
    /// delta t * gamma_X * fission rate 
    /// + delta t * lambda_I * I 
    /// + X^t
    #[inline]
    pub fn calc_xe_135_and_return_conc(
        &mut self,
        timestep: Time,
        fission_rate: VolumetricNumberRate,
        fissioning_nuclide: FissioningNuclideType,
        thermal_neutron_conc: VolumetricNumberDensity,
        ) -> VolumetricNumberDensity {

        let lambda_i = Self::iodine_135_decay_const();
        let lambda_x = Self::xe_135_decay_const();

        let iodine_conc = self.calc_iodine_135_and_return_conc(
            timestep, 
            fission_rate, 
            fissioning_nuclide
        );

        let xe135_addition_rate_from_iodine: VolumetricNumberRate = 
            (iodine_conc * lambda_i).into();

        let gamma_x = match fissioning_nuclide {
            FissioningNuclideType::U233 => Self::fp_yield_xe_135_from_u233_thermal_fission(),
            FissioningNuclideType::U235 => Self::fp_yield_xe_135_from_u235_thermal_fission(),
            FissioningNuclideType::Pu239 => Self::fp_yield_xe_135_from_pu239_thermal_fission(),
        };

        let xe135_addition_rate_from_fission: VolumetricNumberRate = 
            (gamma_x * fission_rate).into();

        let xe135_conc_last_timestep = self.xenon_135_concentration;

        let mut rhs: VolumetricNumberDensity = xe135_conc_last_timestep;
        rhs += VolumetricNumberDensity::into((xe135_addition_rate_from_iodine * timestep).into());
        rhs += VolumetricNumberDensity::into((xe135_addition_rate_from_fission * timestep).into());

        // neutron flux  = n(t) * v
        let thermal_neutron_flux = 
            thermal_neutron_conc * Velocity::new::<meter_per_second>(2200.0);

        let micro_xs_abs_xe_135: Area = Self::xe135_thermal_abs_xs();

        let denominator = 
            Ratio::new::<ratio>(1.0) 
            + lambda_x * timestep
            + thermal_neutron_flux * micro_xs_abs_xe_135 * timestep;

        let xe_conc_next_timestep: VolumetricNumberDensity = 
            (rhs/denominator).into();

        self.xenon_135_concentration = xe_conc_next_timestep;

        return xe_conc_next_timestep;
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
