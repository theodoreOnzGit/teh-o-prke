use std::f64::consts::LN_2;

use ndarray::*;
use ndarray_linalg::Solve;
use uom::ConstZero;
use uom::si::f64::*;
use uom::si::volumetric_number_density::per_cubic_meter;
use uom::si::time::second;
use uom::si::ratio::ratio;

use crate::teh_o_prke_error::TehOPrkeError;

/// Decay Constant is essentially the same units as frequency
pub type DecayConstant = Frequency;
/// SixGroupPRKE
#[derive(Debug,Clone,Copy)]
pub struct SixGroupPRKE {
    /// contains an array for the various half lives 
    /// of the
    decay_constant_array: [DecayConstant;6],
    delayed_fraction_array: [Ratio;6],
    /// determines the set of delayed group constants based on your choice 
    /// of fissile isotope
    pub delayed_group_mode: DelayedGroupMode,

    /// precursor_and_neutron_pop_and_source_array 
    pub precursor_and_neutron_pop_and_source_array: [VolumetricNumberDensity;7],
}

/// different nuclides or fuels have different delayed groups
#[derive(Debug,Clone,Copy)]
pub enum DelayedGroupMode {
    /// chooses the U233 group of delayed constants
    U233,
    /// chooses the U235 group of delayed constants
    U235,
    /// chooses the Pu239 group of delayed constants
    Pu239
}

impl SixGroupPRKE {

    /// returns the next timestep neutron source vector
    ///
    /// also updates the current precursor and concentration vector
    pub fn solve_next_timestep_precursor_concentration_and_neutron_pop_vector(
        &mut self,
        timestep: Time,
        reactivity: Ratio,
        neutron_generation_time: Time,
        background_source_rate: VolumetricNumberRate)
        -> Result<Array1<VolumetricNumberDensity>,TehOPrkeError> {

            // first, construct coefficient matrix

            let coefficient_matrix = self.construct_coefficient_matrix(
                timestep, 
                reactivity, 
                neutron_generation_time);
            // next, unit test to check if the units match 
            {
                let _test_quantity: VolumetricNumberDensity = 
                    (coefficient_matrix[[6,0]]
                    * self.precursor_and_neutron_pop_and_source_array[0]).into();
            }

            // map this to a f64
            // all SI units

            let coefficient_matrix_float: Array2<f64> = coefficient_matrix.map(
                |&coefficient_dimensioned_quantity|{
                    let coefficient_float: f64 = 
                        coefficient_dimensioned_quantity.get::<ratio>();
                    coefficient_float
                }
            );

            let delayed_neutron_precursor_group_1_concentration = self.precursor_and_neutron_pop_and_source_array[0];
            let delayed_neutron_precursor_group_2_concentration = self.precursor_and_neutron_pop_and_source_array[1];
            let delayed_neutron_precursor_group_3_concentration = self.precursor_and_neutron_pop_and_source_array[2];
            let delayed_neutron_precursor_group_4_concentration = self.precursor_and_neutron_pop_and_source_array[3];
            let delayed_neutron_precursor_group_5_concentration = self.precursor_and_neutron_pop_and_source_array[4];
            let delayed_neutron_precursor_group_6_concentration = self.precursor_and_neutron_pop_and_source_array[5];
            let neutron_population_number_density = self.precursor_and_neutron_pop_and_source_array[6];

            let precursor_and_neutron_pop_and_source_array_with_background_source = 
                Self::construct_present_timestep_concentration_and_neutron_pop_vector(
                    delayed_neutron_precursor_group_1_concentration, 
                    delayed_neutron_precursor_group_2_concentration, 
                    delayed_neutron_precursor_group_3_concentration, 
                    delayed_neutron_precursor_group_4_concentration, 
                    delayed_neutron_precursor_group_5_concentration, 
                    delayed_neutron_precursor_group_6_concentration, 
                    neutron_population_number_density, 
                    background_source_rate, timestep);

            // neutron and precursor_and_neutron_pop_and_source_vector 
            // also must be mapped to f64 array

            let precursor_and_neutron_pop_and_source_vector: Array1<f64> 
                = precursor_and_neutron_pop_and_source_array_with_background_source.iter().
                map(|precursor_or_neutron_number_density|{
                    // all SI units
                    let number_density_float: f64 = 
                        precursor_or_neutron_number_density.get::<per_cubic_meter>();
                    number_density_float
                }).collect();

            let precursor_and_neutron_pop_and_source_vector_next_timestep_float: Array1<f64>
                = coefficient_matrix_float.solve(&precursor_and_neutron_pop_and_source_vector)?;

            let precursor_and_neutron_pop_and_source_vector_next_timestep: Array1<VolumetricNumberDensity> 
                = precursor_and_neutron_pop_and_source_vector_next_timestep_float.iter()
                .map(
                    |&precursor_or_neutron_number_density_float|{
                        VolumetricNumberDensity::new::<per_cubic_meter>(
                            precursor_or_neutron_number_density_float
                        )
                }).collect();

            // edit the neutron source vector, 
            // However, need to pivot around the source vector

            self.precursor_and_neutron_pop_and_source_array[0] = 
                precursor_and_neutron_pop_and_source_vector_next_timestep[1];
            self.precursor_and_neutron_pop_and_source_array[1] = 
                precursor_and_neutron_pop_and_source_vector_next_timestep[2];
            self.precursor_and_neutron_pop_and_source_array[2] = 
                precursor_and_neutron_pop_and_source_vector_next_timestep[3];
            self.precursor_and_neutron_pop_and_source_array[3] = 
                precursor_and_neutron_pop_and_source_vector_next_timestep[4];
            self.precursor_and_neutron_pop_and_source_array[4] = 
                precursor_and_neutron_pop_and_source_vector_next_timestep[5];
            self.precursor_and_neutron_pop_and_source_array[5] = 
                precursor_and_neutron_pop_and_source_vector_next_timestep[6];
            self.precursor_and_neutron_pop_and_source_array[6] = 
                precursor_and_neutron_pop_and_source_vector_next_timestep[0];

            // return to environment
            Ok(precursor_and_neutron_pop_and_source_vector_next_timestep)

    }

    /// constructs the vector for delayed neutron precursor concentration 
    /// and neutron population concentration
    ///
    /// note that the first precursor concentration group is that 
    /// with the longest half life
    ///
    pub fn construct_present_timestep_concentration_and_neutron_pop_vector(
        delayed_neutron_precursor_group_1_concentration: VolumetricNumberDensity,
        delayed_neutron_precursor_group_2_concentration: VolumetricNumberDensity,
        delayed_neutron_precursor_group_3_concentration: VolumetricNumberDensity,
        delayed_neutron_precursor_group_4_concentration: VolumetricNumberDensity,
        delayed_neutron_precursor_group_5_concentration: VolumetricNumberDensity,
        delayed_neutron_precursor_group_6_concentration: VolumetricNumberDensity,
        neutron_population_number_density: VolumetricNumberDensity,
        background_source_rate: VolumetricNumberRate,
        timestep: Time
        ) -> 
        Array1<VolumetricNumberDensity>{

            let array_width_and_height = 7;
            let mut precursor_and_neutron_pop_and_source_vector: Array1<VolumetricNumberDensity> = 
                Array::zeros(array_width_and_height);

            precursor_and_neutron_pop_and_source_vector[0] = delayed_neutron_precursor_group_1_concentration;
            precursor_and_neutron_pop_and_source_vector[1] = delayed_neutron_precursor_group_2_concentration;
            precursor_and_neutron_pop_and_source_vector[2] = delayed_neutron_precursor_group_3_concentration;
            precursor_and_neutron_pop_and_source_vector[3] = delayed_neutron_precursor_group_4_concentration;
            precursor_and_neutron_pop_and_source_vector[4] = delayed_neutron_precursor_group_5_concentration;
            precursor_and_neutron_pop_and_source_vector[5] = delayed_neutron_precursor_group_6_concentration;


            let background_source_term: VolumetricNumberDensity = 
                (background_source_rate * timestep).into();

            precursor_and_neutron_pop_and_source_vector[6] = 
                background_source_term
                + neutron_population_number_density;


            precursor_and_neutron_pop_and_source_vector
    }

    /// constructs the matrix required for 
    /// solution of implicit six group PRKE
    pub fn construct_coefficient_matrix(&self,
        timestep: Time,
        reactivity: Ratio,
        neutron_generation_time: Time) -> Array2<Ratio> {

        // preliminaries
        let lambda_1 = self.decay_constant_array[0];
        let lambda_2 = self.decay_constant_array[1];
        let lambda_3 = self.decay_constant_array[2];
        let lambda_4 = self.decay_constant_array[3];
        let lambda_5 = self.decay_constant_array[4];
        let lambda_6 = self.decay_constant_array[5];

        let beta_1 = self.delayed_fraction_array[0];
        let beta_2 = self.delayed_fraction_array[1];
        let beta_3 = self.delayed_fraction_array[2];
        let beta_4 = self.delayed_fraction_array[3];
        let beta_5 = self.delayed_fraction_array[4];
        let beta_6 = self.delayed_fraction_array[5];

        let timestep_to_neutron_generation_time_ratio: Ratio = 
            timestep/neutron_generation_time;

        let total_delayed_fraction: Ratio = 
            self.delayed_fraction_array.clone().into_iter().sum();

        // bottom row coefficient
        let bottom_left_coefficient = Ratio::new::<ratio>(1.0) - 
            timestep_to_neutron_generation_time_ratio*(
                reactivity - total_delayed_fraction);
        let delta_t_lambda_1: Ratio = timestep * lambda_1;
        let delta_t_lambda_2: Ratio = timestep * lambda_2;
        let delta_t_lambda_3: Ratio = timestep * lambda_3;
        let delta_t_lambda_4: Ratio = timestep * lambda_4;
        let delta_t_lambda_5: Ratio = timestep * lambda_5;
        let delta_t_lambda_6: Ratio = timestep * lambda_6;

        let array_width_and_height = 7;

        let mut coefficient_matrix: Array2<Ratio> = 
        Array::zeros((array_width_and_height, array_width_and_height));

        // start changing the bottom row 
        {
            coefficient_matrix[[6,0]] = bottom_left_coefficient;
            coefficient_matrix[[6,1]] = -delta_t_lambda_1;
            coefficient_matrix[[6,2]] = -delta_t_lambda_2;
            coefficient_matrix[[6,3]] = -delta_t_lambda_3;
            coefficient_matrix[[6,4]] = -delta_t_lambda_4;
            coefficient_matrix[[6,5]] = -delta_t_lambda_5;
            coefficient_matrix[[6,6]] = -delta_t_lambda_6;
        }
        // then the left column
        {
            coefficient_matrix[[0,0]] = -timestep_to_neutron_generation_time_ratio*beta_1;
            coefficient_matrix[[1,0]] = -timestep_to_neutron_generation_time_ratio*beta_2;
            coefficient_matrix[[2,0]] = -timestep_to_neutron_generation_time_ratio*beta_3;
            coefficient_matrix[[3,0]] = -timestep_to_neutron_generation_time_ratio*beta_4;
            coefficient_matrix[[4,0]] = -timestep_to_neutron_generation_time_ratio*beta_5;
            coefficient_matrix[[5,0]] = -timestep_to_neutron_generation_time_ratio*beta_6;
        }
        // Lastly, the diagonal
        {
            coefficient_matrix[[0,1]] = Ratio::new::<ratio>(1.0)+delta_t_lambda_1;
            coefficient_matrix[[1,2]] = Ratio::new::<ratio>(1.0)+delta_t_lambda_2;
            coefficient_matrix[[2,3]] = Ratio::new::<ratio>(1.0)+delta_t_lambda_3;
            coefficient_matrix[[3,4]] = Ratio::new::<ratio>(1.0)+delta_t_lambda_4;
            coefficient_matrix[[4,5]] = Ratio::new::<ratio>(1.0)+delta_t_lambda_5;
            coefficient_matrix[[5,6]] = Ratio::new::<ratio>(1.0)+delta_t_lambda_6;
        }

        // return the coefficient_matrix
        coefficient_matrix
    }

    
}


/// produces a new decay constant array
pub fn new_decay_constant_array() -> [DecayConstant;6] {

    // from table 5.1 (not sure where from, duderstadt?)
    // need to double check
    let mut decay_constant_array: [DecayConstant;6] = 
        [
        Frequency::ZERO,
        Frequency::ZERO,
        Frequency::ZERO,
        Frequency::ZERO,
        Frequency::ZERO,
        Frequency::ZERO,
        ];

    let half_life_array: [Time;6] = 
        [
        Time::new::<second>(56.0),
        Time::new::<second>(23.0),
        Time::new::<second>(6.2),
        Time::new::<second>(2.3),
        Time::new::<second>(0.61),
        Time::new::<second>(0.23),
        ];

    // crude, but gets the job done
    //
    // lambda (decay constant) = ln(2)/(half life)
    decay_constant_array[0] = LN_2/half_life_array[0];
    decay_constant_array[1] = LN_2/half_life_array[1];
    decay_constant_array[2] = LN_2/half_life_array[2];
    decay_constant_array[3] = LN_2/half_life_array[3];
    decay_constant_array[4] = LN_2/half_life_array[4];
    decay_constant_array[5] = LN_2/half_life_array[5];

    // done!
    decay_constant_array
        

}

/// produces a new delayed fraction for u233 
pub fn new_u233_delayed_neutron_fraction_array() -> [Ratio;6] {

    let delayed_neutron_array: [Ratio;6] = 
        [
        Ratio::new::<ratio>(0.00023),
        Ratio::new::<ratio>(0.00078),
        Ratio::new::<ratio>(0.00064),
        Ratio::new::<ratio>(0.00074),
        Ratio::new::<ratio>(0.00014),
        Ratio::new::<ratio>(0.00008),
        ];

    delayed_neutron_array
}

/// produces a new delayed fraction for u235 
pub fn new_u235_delayed_neutron_fraction_array() -> [Ratio;6] {

    let delayed_neutron_array: [Ratio;6] = 
        [
        Ratio::new::<ratio>(0.00021),
        Ratio::new::<ratio>(0.00142),
        Ratio::new::<ratio>(0.00128),
        Ratio::new::<ratio>(0.00257),
        Ratio::new::<ratio>(0.00075),
        Ratio::new::<ratio>(0.00027),
        ];

    delayed_neutron_array
}


/// produces a new delayed fraction for pu239 
pub fn new_pu239_delayed_neutron_fraction_array() -> [Ratio;6] {

    let delayed_neutron_array: [Ratio;6] = 
        [
        Ratio::new::<ratio>(0.00007),
        Ratio::new::<ratio>(0.00063),
        Ratio::new::<ratio>(0.00044),
        Ratio::new::<ratio>(0.00069),
        Ratio::new::<ratio>(0.00018),
        Ratio::new::<ratio>(0.00009),
        ];

    delayed_neutron_array
}

#[test]
pub fn test_pu239_total_delayed_frac(){

    let total_delayed_fraction_reference = Ratio::new::<ratio>(0.00210);
    
    let pu239_delayed_frac_array = new_pu239_delayed_neutron_fraction_array();

    let test_delayed_frac: Ratio = pu239_delayed_frac_array.into_iter().sum();

    assert_eq!(test_delayed_frac,total_delayed_fraction_reference)


}

#[test]
pub fn test_u233_total_delayed_frac(){

    let total_delayed_fraction_reference = Ratio::new::<ratio>(0.00261);
    
    let u233_delayed_frac_array = new_u233_delayed_neutron_fraction_array();

    let test_delayed_frac: Ratio = u233_delayed_frac_array.into_iter().sum();

    assert_eq!(test_delayed_frac,total_delayed_fraction_reference)


}

#[test]
pub fn test_u235_total_delayed_frac(){

    use approx::*;
    let total_delayed_fraction_reference = Ratio::new::<ratio>(0.00650);
    
    let u235_delayed_frac_array = new_u235_delayed_neutron_fraction_array();

    let test_delayed_frac: Ratio = u235_delayed_frac_array.into_iter().sum();

    assert_abs_diff_eq!(
        test_delayed_frac.get::<ratio>(), 
        total_delayed_fraction_reference.get::<ratio>(), 
        epsilon = f64::EPSILON);


}
