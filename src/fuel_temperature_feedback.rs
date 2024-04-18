use uom::si::f64::*;
use uom::si::thermodynamic_temperature::kelvin;
use crate::teh_o_prke_error::TehOPrkeError;

/// a struct for calculating fuel temperature feedback
/// using a rather simple heat balance equations
///
/// m c_p (dT_fuel/dt) = -hA(T_fuel-T_surr) + fission_power_source
///
/// uses explicit time stepping for simplicity
///
pub struct SimpleFuelTemperatureFeedback {
    /// m c_p
    pub heat_capacity: HeatCapacity,
    /// T_fuel 
    pub fuel_temperature: ThermodynamicTemperature,
    /// convection heat transfer coefficient
    pub convection_heat_trf_coeff: HeatTransfer,
    /// convection heat transfer area 
    pub convection_heat_trf_area: Area,

}

impl SimpleFuelTemperatureFeedback {
    
    /// add fission heat 
    pub fn add_fission_heat(&mut self,
        fission_power: Power,
        timestep: Time) -> Result<(),TehOPrkeError>{

        todo!()
    }
}


#[inline]
/// for thermal spectrum reactors,
/// the alpha  = d(rho)/dT 
///
/// can be expressed as alpha = -alpha_coefficient/sqrt(T(kelvin))
/// (see lamarsh)
///
/// alpha_coefficient is some value, usually on the order of 1*10^(-4)
/// 
pub fn obtain_fuel_temperature_feedback_coeff_thermal_spectrum(
    alpha_coefficient: Ratio,
    temperature: ThermodynamicTemperature) -> Result<Ratio, TehOPrkeError> {

    let alpha = -alpha_coefficient/
        (temperature.get::<kelvin>().sqrt());

    Ok(alpha)

}


#[inline]
/// for thermal spectrum reactors,
/// we can calculate reactivity based on alpha
///
/// can be expressed as d(rho)/dT = -alpha_coefficient/sqrt(T(kelvin))
///
/// alpha_coefficient is some value, usually on the order of 1*10^(-4)
///
/// now, if you want to have a reactivity with respect to some temperature,
/// we can analytically integrate:
///
/// rho - rho_ref = -alpha_coefficient * 2.0 (sqrt(T) - sqrt(T_ref))
///
/// we can define rho_ref as reactivity at some temperature T_ref
///
/// The function will return (rho - rho_ref), or delta_rho
/// 
///
pub fn obtain_fuel_temperature_reactivity_feedback_thermal_spectrum(
    alpha_coefficient: Ratio,
    temperature: ThermodynamicTemperature,
    reference_temperature: ThermodynamicTemperature) -> Result<Ratio, TehOPrkeError> {

    let delta_rho = -2.0 * alpha_coefficient *
        (temperature.get::<kelvin>().sqrt()
         - reference_temperature.get::<kelvin>().sqrt());

    Ok(delta_rho)

}
