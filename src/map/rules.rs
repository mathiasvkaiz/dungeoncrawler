use ghx_proc_gen::generator::{
    model::ModelCollection,
    rules::{Rules, RulesBuilder},
    socket::SocketCollection,
};
use ghx_proc_gen::ghx_grid::cartesian::coordinates::Cartesian3D;

mod grass_green;
mod grass_yellow;
mod props;
mod water;

pub fn build_rules() -> (Rules<Cartesian3D>, Vec<Option<usize>>) {
    let mut models = ModelCollection::<Cartesian3D>::new();
    let mut sprite_index: Vec<Option<usize>> = Vec::new();
    let mut sockets = SocketCollection::new();

    let (layer1_up, grass_up) = grass_green::gen_con(&mut models, &mut sockets, &mut sprite_index);
    let layer2_up = grass_yellow::gen_con(
        &mut models,
        &mut sockets,
        &mut sprite_index,
        layer1_up,
        grass_up,
    );
    let (layer3_up, ground_up) =
        water::gen_con(&mut models, &mut sockets, &mut sprite_index, layer2_up);
    props::gen_con(
        &mut models,
        &mut sockets,
        &mut sprite_index,
        layer3_up,
        ground_up,
    );

    let rules = RulesBuilder::new_cartesian_3d(models, sockets)
        .build()
        .expect("failed to build WFC rules.");

    (rules, sprite_index)
}