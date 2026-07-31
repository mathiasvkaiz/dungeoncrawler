use ghx_proc_gen::generator::{
    model::ModelCollection, model::ModelRotation, socket::Socket, socket::SocketCollection,
    socket::SocketsCartesian3D,
};
use ghx_proc_gen::ghx_grid::{cartesian::coordinates::Cartesian3D, direction::Direction};

use crate::map::tiles::TileKind;

const UP_AXIS: Direction = Direction::ZForward;
const TREE_WEIGHT: f32 = 0.025;
const PLANTS_WEIGHT: f32 = 0.025;
const STUMPS_WEIGHT: f32 = 0.012;
const ROCKS_WEIGHT: f32 = 0.008;

pub fn gen_con(
    models: &mut ModelCollection<Cartesian3D>,
    sockets: &mut SocketCollection,
    sprite_index: &mut Vec<Option<usize>>,
    layer3_up: Socket,
    ground_up: Socket,
) {
    let void = sockets.create();
    let layer4_ground_down = sockets.create();
    let layer4_water_down = sockets.create();
    let big_tree_upper = sockets.create();
    let big_tree_bottom = sockets.create();

    sockets
        .add_connections(vec![
            (void, vec![void]),
            (big_tree_upper, vec![big_tree_bottom]),
        ])
        .add_rotated_connection(ground_up, vec![layer4_ground_down])
        .add_rotated_connection(layer3_up, vec![layer4_water_down]);

    // layer 4, Void
    models.create(SocketsCartesian3D::Simple {
        x_pos: void,
        x_neg: void,
        y_pos: void,
        y_neg: void,
        z_pos: void,
        z_neg: layer4_water_down,
    });
    sprite_index.push(None);

    let standalone = SocketsCartesian3D::Simple {
        x_pos: void,
        x_neg: void,
        y_pos: void,
        y_neg: void,
        z_pos: void,
        z_neg: layer4_ground_down,
    }
    .to_template();

    // Small Tree
    models.create(standalone.clone().with_weight(TREE_WEIGHT));
    sprite_index.push(Some(TileKind::SmallTree.sprite_index()));

    // Plants
    models.create(standalone.clone().with_weight(PLANTS_WEIGHT));
    sprite_index.push(Some(TileKind::Plant1.sprite_index()));
    models.create(standalone.clone().with_weight(PLANTS_WEIGHT));
    sprite_index.push(Some(TileKind::Plant2.sprite_index()));
    models.create(standalone.clone().with_weight(PLANTS_WEIGHT));
    sprite_index.push(Some(TileKind::Plant3.sprite_index()));

    // Tree Strump
    models.create(standalone.clone().with_weight(STUMPS_WEIGHT));
    sprite_index.push(Some(TileKind::TreeStump1.sprite_index()));
    models.create(standalone.clone().with_weight(STUMPS_WEIGHT));
    sprite_index.push(Some(TileKind::TreeStump2.sprite_index()));
    models.create(standalone.clone().with_weight(STUMPS_WEIGHT));
    sprite_index.push(Some(TileKind::TreeStump3.sprite_index()));

    // Rock
    models.create(standalone.clone().with_weight(ROCKS_WEIGHT));
    sprite_index.push(Some(TileKind::Rock1.sprite_index()));
    models.create(standalone.clone().with_weight(ROCKS_WEIGHT));
    sprite_index.push(Some(TileKind::Rock2.sprite_index()));
    models.create(standalone.clone().with_weight(ROCKS_WEIGHT));
    sprite_index.push(Some(TileKind::Rock3.sprite_index()));
    models.create(standalone.clone().with_weight(ROCKS_WEIGHT));
    sprite_index.push(Some(TileKind::Rock4.sprite_index()));

    let big_tree = SocketsCartesian3D::Simple {
        x_pos: big_tree_upper,
        x_neg: void,
        y_pos: void,
        y_neg: big_tree_bottom,
        z_pos: void,
        z_neg: layer4_ground_down,
    }
    .to_template()
    .with_weight(TREE_WEIGHT);

    models.create(big_tree.clone());
    sprite_index.push(Some(TileKind::BigTreeTL.sprite_index()));
    models.create(big_tree.clone().rotated(ModelRotation::Rot90, UP_AXIS));
    sprite_index.push(Some(TileKind::BigTreeBL.sprite_index()));
    models.create(big_tree.clone().rotated(ModelRotation::Rot180, UP_AXIS));
    sprite_index.push(Some(TileKind::BigTreeBR.sprite_index()));
    models.create(big_tree.clone().rotated(ModelRotation::Rot270, UP_AXIS));
    sprite_index.push(Some(TileKind::BigTreeTR.sprite_index()));
}