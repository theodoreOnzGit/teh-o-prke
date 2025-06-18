use tuas_boussinesq_solver::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component::FluidComponent;
use tuas_boussinesq_solver::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::{FluidComponentCollection, FluidComponentCollectionMethods};
use tuas_boussinesq_solver::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_super_collection::FluidComponentSuperCollection;
use tuas_boussinesq_solver::pre_built_components::non_insulated_fluid_components::NonInsulatedFluidComponent;
use tuas_boussinesq_solver::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use uom::si::f64::*;
use uom::ConstZero;


pub fn get_mass_flowrate_across_for_reactor_downcomers_and_heat_exchg_br(
    pri_loop_branches: &FluidComponentSuperCollection
) -> (MassRate, MassRate, MassRate, MassRate) {



    todo!();

}


/// ciet coupled dracs loop calculations, serves as an example
pub fn ciet_get_abs_mass_flowrate_across_two_branches(dracs_branches: &FluidComponentSuperCollection) -> 
MassRate {
    let pressure_change_across_each_branch = 
        dracs_branches.get_pressure_change(MassRate::ZERO);

    let mass_flowrate_across_each_branch: Vec<MassRate> = 
        dracs_branches.
        get_mass_flowrate_across_each_parallel_branch(
            pressure_change_across_each_branch
        );

    let mut mass_flowrate: MassRate = 
        mass_flowrate_across_each_branch[0];


    // get absolute value
    mass_flowrate = mass_flowrate.abs();

    mass_flowrate

}
/// ciet coupled dracs loop calculations, serves as an example
pub fn ciet_coupled_dracs_fluid_mechanics_calc_abs_mass_rate_no_tchx_calibration(
    pipe_34: &InsulatedFluidComponent,
    pipe_33: &InsulatedFluidComponent,
    pipe_32: &InsulatedFluidComponent,
    pipe_31a: &InsulatedFluidComponent,
    static_mixer_61_label_31: &InsulatedFluidComponent,
    dhx_tube_side_30b: &NonInsulatedFluidComponent,
    dhx_tube_side_heat_exchanger_30: &FluidComponent,
    dhx_tube_side_30a: &NonInsulatedFluidComponent,
    tchx_35a: &NonInsulatedFluidComponent,
    tchx_35b: &NonInsulatedFluidComponent,
    static_mixer_60_label_36: &InsulatedFluidComponent,
    pipe_36a: &InsulatedFluidComponent,
    pipe_37: &InsulatedFluidComponent,
    flowmeter_60_37a: &NonInsulatedFluidComponent,
    pipe_38: &InsulatedFluidComponent,
    pipe_39: &InsulatedFluidComponent,
)-> MassRate {

    let mut dracs_hot_branch = 
        FluidComponentCollection::new_series_component_collection();

    dracs_hot_branch.clone_and_add_component(pipe_34);
    dracs_hot_branch.clone_and_add_component(pipe_33);
    dracs_hot_branch.clone_and_add_component(pipe_32);
    dracs_hot_branch.clone_and_add_component(pipe_31a);
    dracs_hot_branch.clone_and_add_component(static_mixer_61_label_31);
    dracs_hot_branch.clone_and_add_component(dhx_tube_side_30b);
    dracs_hot_branch.clone_and_add_component(dhx_tube_side_heat_exchanger_30);
    dracs_hot_branch.clone_and_add_component(dhx_tube_side_30a);


    let mut dracs_cold_branch = 
        FluidComponentCollection::new_series_component_collection();

    dracs_cold_branch.clone_and_add_component(tchx_35a);
    dracs_cold_branch.clone_and_add_component(tchx_35b);
    dracs_cold_branch.clone_and_add_component(static_mixer_60_label_36);
    dracs_cold_branch.clone_and_add_component(pipe_36a);
    dracs_cold_branch.clone_and_add_component(pipe_37);
    dracs_cold_branch.clone_and_add_component(flowmeter_60_37a);
    dracs_cold_branch.clone_and_add_component(pipe_38);
    dracs_cold_branch.clone_and_add_component(pipe_39);

    let mut dracs_branches = 
        FluidComponentSuperCollection::default();

    dracs_branches.set_orientation_to_parallel();
    dracs_branches.fluid_component_super_vector.push(dracs_hot_branch);
    dracs_branches.fluid_component_super_vector.push(dracs_cold_branch);

    let abs_mass_rate = ciet_get_abs_mass_flowrate_across_two_branches(&dracs_branches);

    abs_mass_rate

}
