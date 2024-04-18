use uom::si::f64::*;
use uom::si::ratio::ratio;
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
    /// rho V c_p = m c_p
    ///
    /// let's do c_p
    pub fuel_specific_heat_capacity: SpecificHeatCapacity,

    /// rho 
    pub fuel_density: MassDensity,

    /// volume 
    pub fuel_volume: Volume,

    /// T_fuel 
    pub fuel_temperature: ThermodynamicTemperature,
    /// convection heat transfer coefficient
    pub convection_heat_trf_coeff: HeatTransfer,
    /// convection heat transfer area 
    pub convection_heat_trf_area: Area,

    /// fuel temperature feedback coefficient
    /// can be expressed as alpha = -alpha_coefficient/sqrt(T(kelvin))
    ///
    /// typically around 10^(-4) dimensionless
    pub alpha_coefficient: Ratio,

}

impl SimpleFuelTemperatureFeedback {
    
    /// add fission heat 
    ///
    ///
    /// m c_p (T_new - T_old)/Delta t = fission power source 
    ///
    /// T_new = (fission power source) * (Delta t) / (m c_p) + T_old
    pub fn add_fission_heat(&mut self,
        fission_power: Power,
        timestep: Time) -> Result<(),TehOPrkeError>{

        let new_temperature = (fission_power * timestep) / 
            (self.fuel_volume * self.fuel_density * self.fuel_specific_heat_capacity)
            + self.fuel_temperature;

        self.fuel_temperature = new_temperature;
        Ok(())
    }

    /// remove heat due to convection
    /// m c_p (dT_fuel/dt) = -hA(T_fuel-T_surr) + fission_power_source
    ///
    /// implicit time stepping used
    ///
    /// m c_p (T_new - T_old)/Delta t = -hA(T_new-T_surr)
    /// T_new - T_old = -hA*(delta t)/(m c_p)*(T_new-T_surr)
    /// T_new - T_old + hA*(delta t)/(m c_p) T_new = hA*(delta t)/(m c_p)(T_surr)
    /// T_new  + hA*(delta t)/(m c_p) T_new = hA*(delta t)/(m c_p)(T_surr) + T_old
    /// T_new  ( 1+ hA*(delta t)/(m c_p) ) = hA*(delta t)/(m c_p)(T_surr) + T_old
    /// T_new  = 1/( 1+ hA*(delta t)/(m c_p) ) * [ hA*(delta t)/(m c_p)(T_surr) + T_old]
    /// 
    ///
    pub fn remove_convection_heat(&mut self,
        coolant_temperature: ThermodynamicTemperature,
        timestep: Time,) -> Result<(),TehOPrkeError>{

        let m_cp = self.fuel_volume * self.fuel_density * self.fuel_specific_heat_capacity;

        let h_a_delta_t_by_mcp: Ratio = 
            self.convection_heat_trf_coeff * 
            self.convection_heat_trf_area * 
            timestep / 
            m_cp;

        let numerator: ThermodynamicTemperature = 
            h_a_delta_t_by_mcp * coolant_temperature + self.fuel_temperature;

        let denominator: Ratio = Ratio::new::<ratio>(1.0) + h_a_delta_t_by_mcp;

        let new_temperature: ThermodynamicTemperature = 
            numerator/(denominator.get::<ratio>());


        self.fuel_temperature = new_temperature;

        Ok(())
    }

    /// obtain reactivity change compared to reference temperature 
    /// usually 300K
    pub fn obtain_fuel_temperature_delta_rho(&self,
        reference_temperature: ThermodynamicTemperature,) -> Result<Ratio, TehOPrkeError>{

        let alpha_coefficient = self.alpha_coefficient;
        let temperature = self.fuel_temperature;

        obtain_fuel_temperature_reactivity_feedback_thermal_spectrum(
            alpha_coefficient, temperature, reference_temperature)

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
