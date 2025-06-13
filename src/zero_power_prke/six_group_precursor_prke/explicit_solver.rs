use ndarray::*;
use ndarray_linalg::Solve;
use uom::si::f64::*;
use uom::si::volumetric_number_density::per_cubic_meter;
use uom::si::ratio::ratio;

use crate::teh_o_prke_error::TehOPrkeError;
use crate::time_stepping::openfoam_ode_system::ODESystem;
use super::SixGroupPRKE;
impl SixGroupPRKE {
    pub fn solve_next_timestep_precursor_concentration_and_neutron_pop_vector_explicit(
        &mut self,
        timestep: Time,
        reactivity: Ratio,
        neutron_generation_time: Time,
        background_source_rate: VolumetricNumberRate)
        -> Result<Array1<VolumetricNumberDensity>,TehOPrkeError> {


            // first, get neutron population and precursor density
            let neutron_population_number_density = self.precursor_and_neutron_pop_and_source_array[0];
            let delayed_neutron_precursor_group_1_concentration = self.precursor_and_neutron_pop_and_source_array[1];
            let delayed_neutron_precursor_group_2_concentration = self.precursor_and_neutron_pop_and_source_array[2];
            let delayed_neutron_precursor_group_3_concentration = self.precursor_and_neutron_pop_and_source_array[3];
            let delayed_neutron_precursor_group_4_concentration = self.precursor_and_neutron_pop_and_source_array[4];
            let delayed_neutron_precursor_group_5_concentration = self.precursor_and_neutron_pop_and_source_array[5];
            let delayed_neutron_precursor_group_6_concentration = self.precursor_and_neutron_pop_and_source_array[6];

            // construct the ODE system 
            let ode_system: ODESystem;

            // the ode system here takes a time t 
            // and a vector of neutron population and precursor densities 
            // and then returns the vector of derivatives
            // that is 
            //
            // dn/dt = (rho - beta)/Lambda n(t) + sum_i^n lambda_i C_i 
            // d C_i/dt = beta_i/Lambda n(t) - lambda_i C_i
            todo!()
    }

}
