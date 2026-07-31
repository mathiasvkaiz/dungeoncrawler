use ghx_proc_gen::generator::{
    model::ModelCollection, model::ModelRotation, socket::Socket, socket::SocketCollection,
    socket::SocketsCartesian3D,
};
use ghx_proc_gen::ghx_grid::{cartesian::coordinates::Cartesian3D, direction::Direction};

use crate::map::tiles::TileKind;

const UP_AXIS: Direction = Direction::ZForward;
const WATER_WEIGHT: f32 = 0.005;

pub fn gen_con(
    models: &mut ModelCollection<Cartesian3D>,
    sockets: &mut SocketCollection,
    sprite_index: &mut Vec<Option<usize>>,
    layer2_up: Socket,
) -> (Socket, Socket) {
    let layer3_up = sockets.create();
    let layer3_down = sockets.create();
    let ground_up = sockets.create();
    let water = sockets.create();
    let water_and_void = sockets.create();
    let void_and_water = sockets.create();
    let void = sockets.create();

    sockets
        .add_connections(vec![
            (water, vec![water]),
            (void, vec![void]),
            (water_and_void, vec![void_and_water]),
            (void_and_water, vec![water_and_void]),
        ])
        .add_rotated_connection(layer2_up, vec![layer3_down]);

    // layer 3, Void And Water
    models.create(SocketsCartesian3D::Multiple {
        x_pos: vec![void],
        x_neg: vec![void],
        y_pos: vec![void],
        y_neg: vec![void],
        z_pos: vec![layer3_up, ground_up],
        z_neg: vec![layer3_down],
    });
    sprite_index.push(None);
    models
        .create(SocketsCartesian3D::Simple {
            x_pos: water,
            x_neg: water,
            y_pos: water,
            y_neg: water,
            z_pos: layer3_up,
            z_neg: layer3_down,
        })
        .with_weight(10. * WATER_WEIGHT);
    sprite_index.push(Some(TileKind::Water.sprite_index()));

    // Grass Side, Corner and Inside

    let water_side = SocketsCartesian3D::Simple {
        x_pos: void_and_water,
        x_neg: water_and_void,
        y_pos: void,
        y_neg: water,
        z_pos: layer3_up,
        z_neg: layer3_down,
    }
    .to_template()
    .with_weight(WATER_WEIGHT);

    let water_cor = SocketsCartesian3D::Simple {
        x_pos: void_and_water,
        x_neg: void,
        y_pos: void,
        y_neg: water_and_void,
        z_pos: layer3_up,
        z_neg: layer3_down,
    }
    .to_template()
    .with_weight(WATER_WEIGHT);

    let water_in = SocketsCartesian3D::Simple {
        x_pos: water_and_void,
        x_neg: water,
        y_pos: water,
        y_neg: void_and_water,
        z_pos: layer3_up,
        z_neg: layer3_down,
    }
    .to_template()
    .with_weight(WATER_WEIGHT);

    // Side
    models.create(water_side.clone());
    sprite_index.push(Some(TileKind::WaterSideT.sprite_index()));
    models.create(water_side.clone().rotated(ModelRotation::Rot90, UP_AXIS));
    sprite_index.push(Some(TileKind::WaterSideL.sprite_index()));
    models.create(water_side.clone().rotated(ModelRotation::Rot180, UP_AXIS));
    sprite_index.push(Some(TileKind::WaterSideB.sprite_index()));
    models.create(water_side.clone().rotated(ModelRotation::Rot270, UP_AXIS));
    sprite_index.push(Some(TileKind::WaterSideR.sprite_index()));

    // Corner
    models.create(water_cor.clone());
    sprite_index.push(Some(TileKind::WaterCornerTL.sprite_index()));
    models.create(water_cor.clone().rotated(ModelRotation::Rot90, UP_AXIS));
    sprite_index.push(Some(TileKind::WaterCornerBL.sprite_index()));
    models.create(water_cor.clone().rotated(ModelRotation::Rot180, UP_AXIS));
    sprite_index.push(Some(TileKind::WaterCornerBR.sprite_index()));
    models.create(water_cor.clone().rotated(ModelRotation::Rot270, UP_AXIS));
    sprite_index.push(Some(TileKind::WaterCornerTR.sprite_index()));

    // Inside
    models.create(water_in.clone());
    sprite_index.push(Some(TileKind::WaterInsideTL.sprite_index()));
    models.create(water_in.clone().rotated(ModelRotation::Rot90, UP_AXIS));
    sprite_index.push(Some(TileKind::WaterInsideBL.sprite_index()));
    models.create(water_in.clone().rotated(ModelRotation::Rot180, UP_AXIS));
    sprite_index.push(Some(TileKind::WaterInsideBR.sprite_index()));
    models.create(water_in.clone().rotated(ModelRotation::Rot270, UP_AXIS));
    sprite_index.push(Some(TileKind::WaterInsideTR.sprite_index()));

    (layer3_up, ground_up)
}