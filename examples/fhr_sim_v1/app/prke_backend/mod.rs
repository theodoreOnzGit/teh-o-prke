use std::f64::consts::LN_2;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use teh_o_prke::{feedback_mechanisms::SixFactorFormulaFeedback, zero_power_prke::six_group::SixGroupPRKE};
use uom::si::available_energy::joule_per_kilogram;
use uom::si::energy::megaelectronvolt;
use uom::si::linear_number_density::per_meter;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::time::{day, hour, second};
use uom::si::velocity::meter_per_second;
use uom::si::volume::cubic_meter;
use uom::si::volumetric_number_rate::per_cubic_meter_second;
use uom::si::{f64::*, ratio::ratio};
use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
use uom::ConstZero;

use crate::{FHRSimulatorApp, FHRState};

impl FHRSimulatorApp {

    /// associated function for PRKE calculation 
    /// for continuous loop
    pub fn calculate_prke_loop(
        fhr_state: Arc<Mutex<FHRState>>){

        // construct a prke six group object
        // probably want to use a u235 group or u233 group
        // default
        let mut prke_six_group :SixGroupPRKE = SixGroupPRKE::default();
        // now this is arbitrary, user can set
        let mut keff_six_factor = SixFactorFormulaFeedback::default();
        // start from keff = 1
        // all terms = 1
        // start with leakage
        keff_six_factor.p_tnl = Ratio::new::<ratio>(0.9);
        keff_six_factor.p_fnl = Ratio::new::<ratio>(0.7);
        // then fuel reproduction
        keff_six_factor.eta = Ratio::new::<ratio>(2.2);
        // resonance esc probability 
        keff_six_factor.p = Ratio::new::<ratio>(0.8);
        // thermal utilisation 
        keff_six_factor.f = Ratio::new::<ratio>(0.9);
        // fast fission 
        keff_six_factor.epsilon = Ratio::new::<ratio>(1.03);
        // keff total is about 1.0278
        // excess reactivity is about 0.0278
        //
        // basically control rod should be about this much 
        // and fuel temp feedback about this much also
        // some of these are arbitrary

        let timestep = Time::new::<second>(1.0e-4);
        let reactor_volume = Volume::new::<cubic_meter>(0.5);
        let macroscopic_fission_xs = LinearNumberDensity::new::<per_meter>(1.0);
        let mut fhr_state_clone = fhr_state.lock().unwrap().clone();

        // then decay heat struct 
        let mut fhr_decay_heat = FHRDecayHeat::default();

        loop {
            Self::calculate_prke_for_one_timestep(&mut fhr_state_clone,
                &mut keff_six_factor,
                &mut prke_six_group,
                timestep,
                reactor_volume,
                macroscopic_fission_xs,
                &mut fhr_decay_heat,
            );
            let dur = Duration::from_millis(40);
            thread::sleep(dur);
        }

    }
    /// associated function for PRKE calculation 
    /// for single timestep
    pub fn calculate_prke_for_one_timestep(
        fhr_state_ref: &mut FHRState,
        keff_six_factor: &mut SixFactorFormulaFeedback,
        prke_six_group: &mut SixGroupPRKE,
        timestep: Time,
        reactor_volume: Volume,
        macroscopic_fission_xs: LinearNumberDensity,
        fhr_decay_heat: &mut FHRDecayHeat,
        ){

        // within each timestep, I need to obtain feedback
        // so basically for now, fuel temperature and control rod insertion 
        // based on that, calculate PRKE


        let fuel_temp: ThermodynamicTemperature = 
            ThermodynamicTemperature::new::<degree_celsius>(
                fhr_state_ref.pebble_core_temp_degc
            );

        let left_cr_insertion_frac = 
            fhr_state_ref.left_cr_insertion_frac;
        let right_cr_insertion_frac = 
            fhr_state_ref.right_cr_insertion_frac;

        // now based on this, calculate feedback

        // after feedback we should get the reactivity 
        let reactivity: Ratio = keff_six_factor.calc_rho();
        let neutron_generation_time = Time::new::<second>(1.0e-4);
        let background_source_rate = 
            VolumetricNumberRate::new::<per_cubic_meter_second>(5.0);



        let _neutron_pop_and_six_group_precursor_vec = 
            prke_six_group.solve_next_timestep_precursor_concentration_and_neutron_pop_vector(
                timestep, 
                reactivity, 
                neutron_generation_time, 
                background_source_rate
            );

        // then get the current neutron population 
        let current_neutron_pop_density: VolumetricNumberDensity = 
            prke_six_group.get_current_neutron_population_density();
        // then total reactor volume to get neutron number 
        //let current_neutron_pop: Ratio =  
        //    reactor_volume * current_neutron_pop_density;
        

        // we can get a power production 
        // and some of it should go to decay heat
        // so we need Sigma_f * phi 
        let neutron_speed: Velocity = Velocity::new::<meter_per_second>(2200.0);
        let current_neutron_flux: ArealNumberRate = 
            (current_neutron_pop_density * neutron_speed).into();

        // then fission rate
        // should be a number Rate
        // per unit vol
        let fission_rate_density: VolumetricNumberRate = 
            (current_neutron_flux * macroscopic_fission_xs).into();

        // should be a frequency
        let fission_rate: Frequency  = 
            fission_rate_density * reactor_volume;

        let power_per_fission = 
            Energy::new::<megaelectronvolt>(200.0);

        // immediate power from fission
        let mut fission_power: Power = 
            power_per_fission * fission_rate;

        // add to decay heat precursors 
        fhr_decay_heat.add_decay_heat_precursor1(fission_power * 0.04);
        fhr_decay_heat.add_decay_heat_precursor2(fission_power * 0.04);
        fhr_decay_heat.add_decay_heat_precursor3(fission_power * 0.02);


        // adjust fission power for decay heat 
        // fission power less decay heat = 1.0 - 0.04 - 0.04 - 0.02 = 0.9
        fission_power *= 0.9;
        fission_power += fhr_decay_heat.calc_decay_heat_power_1(timestep);
        fission_power += fhr_decay_heat.calc_decay_heat_power_2(timestep);
        fission_power += fhr_decay_heat.calc_decay_heat_power_3(timestep);

        // with the correct fission power now, we can 
        // calc temperature
        //


    }

    
    
}


/// this struct helps to manage decay heat calculations
///
/// similar to how delayed neutron precursors are handled,
/// I have some groups for decay heat precursors. 
/// These are loosely coupled (semi-implicit)
#[derive(Clone, Debug, Copy)]
pub struct FHRDecayHeat {
    decay_heat_precursor1: Power,
    pub decay_heat_precursor1_half_life: Time,
    decay_heat_precursor2: Power,
    pub decay_heat_precursor2_half_life: Time,
    decay_heat_precursor3: Power,
    pub decay_heat_precursor3_half_life: Time,
}

impl FHRDecayHeat {
    pub fn add_decay_heat_precursor1(&mut self, 
        decay_precursor_power: Power){

        self.decay_heat_precursor1 += decay_precursor_power;
    }
    pub fn add_decay_heat_precursor2(&mut self, 
        decay_precursor_power: Power){

        self.decay_heat_precursor2 += decay_precursor_power;
    }
    pub fn add_decay_heat_precursor3(&mut self, 
        decay_precursor_power: Power){

        self.decay_heat_precursor3 += decay_precursor_power;
    }


    /// basically 
    ///
    /// (P_decay^(t + Delta t) - P_decay^t)/ Delta t = - lambda_i P_decay^(t + Delta t)
    pub fn calc_decay_heat_power_1(&mut self,
        timestep: Time) -> Power {

        let decay_constant: Frequency = 
            LN_2/self.decay_heat_precursor1_half_life;
        // (P_decay^(t + Delta t) - P_decay^t)/ Delta t = - lambda_i P_decay^(t + Delta t)
        //
        // P_decay^(t + Delta t) (1 + lambda_i * Delta t) = P_decay^t
        //

        let p_decay_t = self.decay_heat_precursor1;
        let coeff = Ratio::new::<ratio>(1.0) + decay_constant * timestep;

        let p_decay_t_plus_delta_t = p_decay_t / coeff;

        self.decay_heat_precursor1 = p_decay_t_plus_delta_t;

        return p_decay_t_plus_delta_t;

        

    }
    /// basically 
    ///
    /// (P_decay^(t + Delta t) - P_decay^t)/ Delta t = - lambda_i P_decay^(t + Delta t)
    pub fn calc_decay_heat_power_2(&mut self,
        timestep: Time) -> Power {

        let decay_constant: Frequency = 
            LN_2/self.decay_heat_precursor2_half_life;
        // (P_decay^(t + Delta t) - P_decay^t)/ Delta t = - lambda_i P_decay^(t + Delta t)
        //
        // P_decay^(t + Delta t) (1 + lambda_i * Delta t) = P_decay^t
        //

        let p_decay_t = self.decay_heat_precursor2;
        let coeff = Ratio::new::<ratio>(1.0) + decay_constant * timestep;

        let p_decay_t_plus_delta_t = p_decay_t / coeff;

        self.decay_heat_precursor2 = p_decay_t_plus_delta_t;

        return p_decay_t_plus_delta_t;

        

    }
    /// basically 
    ///
    /// (P_decay^(t + Delta t) - P_decay^t)/ Delta t = - lambda_i P_decay^(t + Delta t)
    pub fn calc_decay_heat_power_3(&mut self,
        timestep: Time) -> Power {

        let decay_constant: Frequency = 
            LN_2/self.decay_heat_precursor3_half_life;
        // (P_decay^(t + Delta t) - P_decay^t)/ Delta t = - lambda_i P_decay^(t + Delta t)
        //
        // P_decay^(t + Delta t) (1 + lambda_i * Delta t) = P_decay^t
        //

        let p_decay_t = self.decay_heat_precursor3;
        let coeff = Ratio::new::<ratio>(1.0) + decay_constant * timestep;

        let p_decay_t_plus_delta_t = p_decay_t / coeff;

        self.decay_heat_precursor3 = p_decay_t_plus_delta_t;

        return p_decay_t_plus_delta_t;

        

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
impl Default for FHRDecayHeat {
    fn default() -> Self {

        Self { 
            decay_heat_precursor1: Power::ZERO, 
            decay_heat_precursor1_half_life: Time::new::<hour>(0.2), 
            decay_heat_precursor2: Power::ZERO, 
            decay_heat_precursor2_half_life: Time::new::<hour>(8.0), 
            decay_heat_precursor3: Power::ZERO, 
            decay_heat_precursor3_half_life: Time::new::<day>(30.0),
        }
    }
}


/// this struct helps to manage pebble bed thermal hydraulics 
/// calculations
///
/// This is important for fast feedback of fuel temperature 
///
/// basically it keeps track of the thermal inertia of the system
/// among other things
///
#[derive(Clone, Debug, Copy)]
pub struct PebbleBedThermalHydraulics {
    /// this is the current specific enthalpy of the pebble bed
    pub current_fuel_specific_enthalpy: AvailableEnergy,
}

impl PebbleBedThermalHydraulics {

    /// creates a new instance of PebbleBedThermalHydraulics
    pub fn new() -> Self {
        // let's start pebble bed at 500 degc 
        let start_temp = ThermodynamicTemperature::new::<degree_celsius>(500.0);
        let start_enthalpy = Self::get_enthalpy_from_temperature_uo2(start_temp);

        return Self { current_fuel_specific_enthalpy: start_enthalpy };
    }

    /// Carbajo, J. J. (2001). Thermophysical properties of MOX and UO2 
    /// fuels including the effects of irradiation (No. ORNL/TM-2000/351). 
    /// Oak Ridge National Lab.(ORNL), Oak Ridge, TN (United States).
    ///
    /// enthalpy at 298K is deemd to be zero J/kg
    pub fn get_enthalpy_from_temperature_uo2(
        fuel_temp: ThermodynamicTemperature
    ) -> AvailableEnergy {

        // see page 18
        // table 4.2
        // for uranium oxide

        let c1 = SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
            302.27
        );
        let c2 = SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
            8.463e-3
        ) / TemperatureInterval::new::<uom::si::temperature_interval::kelvin>(1.0);
        let c3 = AvailableEnergy::new::<joule_per_kilogram>(
            8.741e7
        );

        // einstein temp
        let theta = ThermodynamicTemperature::new::<kelvin>(
            548.68
        );

        // electron activation energy divide by boltzmann constant 
        let e_a = ThermodynamicTemperature::new::<kelvin>(
            18_531.7
        );

        // some terms for calc 

        let ref_temp_298 = ThermodynamicTemperature::new::<kelvin>(298.0);
        let theta_by_t: Ratio = theta/fuel_temp;
        let theta_by_298: Ratio = theta/ref_temp_298;

        let e_a_by_t: Ratio = e_a/fuel_temp;

        // calc term by term
        let term1: AvailableEnergy = c1 * theta * (
            (theta_by_t.get::<ratio>().exp() - 1.0).recip()
            -(theta_by_298.get::<ratio>().exp() - 1.0).recip()
        );

        let term2: AvailableEnergy = 
            c2 * (fuel_temp * fuel_temp - ref_temp_298 * ref_temp_298);

        let term3: AvailableEnergy = c3 * (-e_a_by_t).exp();

        // enthalpy increment over 298K 
        let enthalpy_increment_over_298_kelvin = 
            term1 + term2 + term3;

        return enthalpy_increment_over_298_kelvin;




    }

    pub fn get_current_temeperature(&self) -> ThermodynamicTemperature {
        todo!()
    }
}
