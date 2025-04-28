use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use teh_o_prke::{feedback_mechanisms::SixFactorFormulaFeedback, zero_power_prke::six_group::SixGroupPRKE};
use uom::si::area::square_meter;
use uom::si::energy::{kilojoule, megaelectronvolt};
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::linear_number_density::per_meter;
use uom::si::mass::kilogram;
use uom::si::time::second;
use uom::si::velocity::meter_per_second;
use uom::si::volume::cubic_meter;
use uom::si::volumetric_number_rate::per_cubic_meter_second;
use uom::si::{f64::*, ratio::ratio};
use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};

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
        let mut pebble_bed_th_struct = 
            PebbleBedThermalHydraulics::new();
        let fhr_state_clone = fhr_state.clone();

        // then decay heat struct 
        let mut fhr_decay_heat = FHRDecayHeat::default();

        loop {
            Self::calculate_prke_for_one_timestep(
                &mut fhr_state_clone.lock().unwrap(),
                &mut keff_six_factor,
                &mut prke_six_group,
                timestep,
                reactor_volume,
                macroscopic_fission_xs,
                &mut fhr_decay_heat,
                &mut pebble_bed_th_struct,
            );
            let dur = Duration::from_millis(40);
            thread::sleep(dur);
        }

    }
    /// associated function for PRKE calculation 
    /// for single timestep
    /// for prke anyway
    /// note that the prke timestep will be different from 
    /// the main thermal hydraulics timestep
    pub fn calculate_prke_for_one_timestep(
        fhr_state_ref: &mut FHRState,
        keff_six_factor: &mut SixFactorFormulaFeedback,
        prke_six_group: &mut SixGroupPRKE,
        prke_timestep: Time,
        reactor_volume: Volume,
        macroscopic_fission_xs: LinearNumberDensity,
        fhr_decay_heat: &mut FHRDecayHeat,
        pebble_bed_th_struct: &mut PebbleBedThermalHydraulics,
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

        keff_six_factor.fuel_temp_feedback(fuel_temp, 
            FHRSimulatorApp::fuel_temp_resonance_esc_feedback_linear);

        // after feedback we should get the reactivity 
        let reactivity: Ratio = keff_six_factor.calc_rho();
        let neutron_generation_time = Time::new::<second>(1.0e-4);
        let background_source_rate = 
            VolumetricNumberRate::new::<per_cubic_meter_second>(5.0);



        let _neutron_pop_and_six_group_precursor_vec = 
            prke_six_group.solve_next_timestep_precursor_concentration_and_neutron_pop_vector(
                prke_timestep, 
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
        fission_power += fhr_decay_heat.calc_decay_heat_power_1(prke_timestep);
        fission_power += fhr_decay_heat.calc_decay_heat_power_2(prke_timestep);
        fission_power += fhr_decay_heat.calc_decay_heat_power_3(prke_timestep);

        // with the correct fission power now, we can 
        // calc temperature
        //
        //
        // These are arbitrary values, will adjust later

        let pebble_bed_mass = Mass::new::<kilogram>(50.0);
        let pebble_bed_heat_transfer_area = Area::new::<square_meter>(20.0);
        let pebble_bed_overall_htc = HeatTransfer::new::<watt_per_square_meter_kelvin>(400.0);
        let pebble_bed_coolant_temp = ThermodynamicTemperature::new::<degree_celsius>(
            fhr_state_ref.pebble_bed_coolant_temp_degc
        );

        let heat_removal_from_pebble_bed = 
            pebble_bed_th_struct.calc_th_and_return_heat_removal_from_pebble_bed(
                prke_timestep, 
                fission_power, 
                pebble_bed_mass, 
                pebble_bed_heat_transfer_area, 
                pebble_bed_overall_htc, 
                pebble_bed_coolant_temp);

        let pebble_bed_fuel_temp = pebble_bed_th_struct.get_temperature_from_enthalpy_uo2_heuristic();

        // update the fhr state 
        fhr_state_ref.pebble_core_temp_degc = 
            pebble_bed_fuel_temp.get::<degree_celsius>();

        // the heat removal from pebble bed should be stored in fhr 
        // state, so as to sync correctly with the coolant
        // the prke timestep will also be added so as to obtain an 
        // average heat removal rate to be transferred to the coolant

        fhr_state_ref.prke_loop_accumulated_heat_removal_kilojoules
            += (heat_removal_from_pebble_bed*prke_timestep).get::<kilojoule>();
        fhr_state_ref.prke_loop_accumulated_timestep_seconds
            += prke_timestep.get::<second>();

        // that settles thermal hydraulics


    }

    /// fuel temperature feedback and how it deals with resonance escape 
    /// probability 
    ///
    /// basically, with increased fuel temp, lower resonance 
    /// esc probability
    ///
    /// this is a heuristic, can be replaced by more complex 
    /// functions later
    ///
    /// the ratio u return is factor of increase or decrease in resonance 
    /// escape probability
    pub fn fuel_temp_resonance_esc_feedback_linear(fuel_temp: ThermodynamicTemperature) -> Ratio {

        let fuel_temp_degc = fuel_temp.get::<degree_celsius>();
        // from an arbitrary map of:
        // fuel_temp_degc, resonance esc change factor
        // 200,1.2,
        // 300,1.1,
        // 400,1.05,
        // 500,1,
        // 600,0.99,
        // 700,0.98,
        // 800,0.97,
        // 900,0.96,
        // 1000,0.95,
        // 1100,0.94,
        // 1200,0.9,
        // 1300,0.85,
        // 1400,0.83,
        // 1500,0.8,
        //
        // I get a trendline in LibreOffice

        return Ratio::new::<ratio>(-2.4220e-4 * fuel_temp_degc + 1.1716e0);


    }
    
    
}

pub mod decay_heat;
pub use decay_heat::*;


pub mod pebble_bed_thermal_hydraulics;
pub use pebble_bed_thermal_hydraulics::*;


