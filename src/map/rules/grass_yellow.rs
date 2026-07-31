use ghx_proc_gen::generator::{
    model::ModelCollection, model::ModelRotation, socket::Socket, socket::SocketCollection,
    socket::SocketsCartesian3D,
};
use ghx_proc_gen::ghx_grid::{cartesian::coordinates::Cartesian3D, direction::Direction};

use crate::map::tiles::TileKind;

const UP_AXIS: Direction = Direction::ZForward;

pub fn gen_con(
    models: &mut ModelCollection<Cartesian3D>,
    sockets: &mut SocketCollection,
    sprite_index: &mut Vec<Option<usize>>,
    layer1_up: Socket,
    grass_up: Socket,
) -> Socket {
    let layer2_void_down = sockets.create();
    let layer2_grass_down = sockets.create();
    let layer2_up = sockets.create();
    let void = sockets.create();
    let grass = sockets.create();
    let grass_and_void = sockets.create();
    let void_and_grass = sockets.create();

    sockets
        .add_connections(vec![
            (grass, vec![grass]),
            (void, vec![void]),
            (grass_and_void, vec![void_and_grass]),
            (void_and_grass, vec![grass_and_void]),
        ])
        .add_rotated_connection(layer1_up, vec![layer2_void_down])
        .add_rotated_connection(grass_up, vec![layer2_grass_down]);

    // layer up, Void And Grass
    models.create(SocketsCartesian3D::Simple {
        x_pos: void,
        x_neg: void,
        y_pos: void,
        y_neg: void,
        z_pos: layer2_up,
        z_neg: layer2_void_down,
    });
    sprite_index.push(None);
    models
        .create(SocketsCartesian3D::Simple {
            x_pos: grass,
            x_neg: grass,
            y_pos: grass,
            y_neg: grass,
            z_pos: layer2_up,
            z_neg: layer2_grass_down,
        })
        .with_weight(3.);
    sprite_index.push(Some(TileKind::YellowGrass.sprite_index()));

    // Grass Side, Corner and Inside

    let grass_side = SocketsCartesian3D::Simple {
        x_pos: void_and_grass,
        x_neg: grass_and_void,
        y_pos: void,
        y_neg: grass,
        z_pos: layer2_up,
        z_neg: layer2_grass_down,
    }
    .to_template();

    let grass_cor = SocketsCartesian3D::Simple {
        x_pos: void_and_grass,
        x_neg: void,
        y_pos: void,
        y_neg: grass_and_void,
        z_pos: layer2_up,
        z_neg: layer2_grass_down,
    }
    .to_template();

    let grass_in = SocketsCartesian3D::Simple {
        x_pos: grass_and_void,
        x_neg: grass,
        y_pos: grass,
        y_neg: void_and_grass,
        z_pos: layer2_up,
        z_neg: layer2_grass_down,
    }
    .to_template();

    // Side
    models.create(grass_side.clone());
    sprite_index.push(Some(TileKind::YellowGrassSideT.sprite_index()));
    models.create(grass_side.clone().rotated(ModelRotation::Rot90, UP_AXIS));
    sprite_index.push(Some(TileKind::YellowGrassSideL.sprite_index()));
    models.create(grass_side.clone().rotated(ModelRotation::Rot180, UP_AXIS));
    sprite_index.push(Some(TileKind::YellowGrassSideB.sprite_index()));
    models.create(grass_side.clone().rotated(ModelRotation::Rot270, UP_AXIS));
    sprite_index.push(Some(TileKind::YellowGrassSideR.sprite_index()));

    // Corner
    models.create(grass_cor.clone());
    sprite_index.push(Some(TileKind::YellowGrassCornerTL.sprite_index()));
    models.create(grass_cor.clone().rotated(ModelRotation::Rot90, UP_AXIS));
    sprite_index.push(Some(TileKind::YellowGrassCornerBL.sprite_index()));
    models.create(grass_cor.clone().rotated(ModelRotation::Rot180, UP_AXIS));
    sprite_index.push(Some(TileKind::YellowGrassCornerBR.sprite_index()));
    models.create(grass_cor.clone().rotated(ModelRotation::Rot270, UP_AXIS));
    sprite_index.push(Some(TileKind::YellowGrassCornerTR.sprite_index()));

    // Inside
    models.create(grass_in.clone());
    sprite_index.push(Some(TileKind::YellowGrassInsideTL.sprite_index()));
    models.create(grass_in.clone().rotated(ModelRotation::Rot90, UP_AXIS));
    sprite_index.push(Some(TileKind::YellowGrassInsideBL.sprite_index()));
    models.create(grass_in.clone().rotated(ModelRotation::Rot180, UP_AXIS));
    sprite_index.push(Some(TileKind::YellowGrassInsideBR.sprite_index()));
    models.create(grass_in.clone().rotated(ModelRotation::Rot270, UP_AXIS));
    sprite_index.push(Some(TileKind::YellowGrassInsideTR.sprite_index()));

    layer2_up
}