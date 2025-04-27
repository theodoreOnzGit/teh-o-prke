use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use teh_o_prke::{feedback_mechanisms::SixFactorFormulaFeedback, zero_power_prke::six_group::SixGroupPRKE};
use uom::si::energy::megaelectronvolt;
use uom::si::linear_number_density::per_meter;
use uom::si::time::second;
use uom::si::velocity::meter_per_second;
use uom::si::volume::cubic_meter;
use uom::si::volumetric_number_rate::per_cubic_meter_second;
use uom::si::{f64::*, ratio::ratio};
use uom::si::thermodynamic_temperature::degree_celsius;

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
        loop {
            Self::calculate_prke_for_one_timestep(&mut fhr_state_clone,
                &mut keff_six_factor,
                &mut prke_six_group,
                timestep,
                reactor_volume,
                macroscopic_fission_xs,
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
        let fission_power: Power = 
            power_per_fission * fission_rate;



    }

    
    
}
