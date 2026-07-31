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
) -> (Socket, Socket) {
    let dirt = sockets.create();
    let layer0_up = sockets.create();
    let layer1_up = sockets.create();
    let layer1_down = sockets.create();
    let grass_up = sockets.create();
    let void = sockets.create();
    let grass = sockets.create();
    let grass_and_void = sockets.create();
    let void_and_grass = sockets.create();

    sockets
        .add_connections(vec![
            (dirt, vec![dirt]),
            (grass, vec![grass]),
            (void, vec![void]),
            (grass_and_void, vec![void_and_grass]),
            (void_and_grass, vec![grass_and_void]),
        ])
        .add_rotated_connection(layer0_up, vec![layer1_down]);

    // layer 0, Dirt
    models.create(SocketsCartesian3D::Simple {
        x_pos: dirt,
        x_neg: dirt,
        y_pos: dirt,
        y_neg: dirt,
        z_pos: layer0_up,
        z_neg: dirt,
    });
    sprite_index.push(Some(TileKind::Dirt.sprite_index()));
    // layer 1, Void And Grass
    models.create(SocketsCartesian3D::Simple {
        x_pos: void,
        x_neg: void,
        y_pos: void,
        y_neg: void,
        z_pos: void,
        z_neg: layer1_down,
    });
    sprite_index.push(None);
    models
        .create(SocketsCartesian3D::Multiple {
            x_pos: vec![grass],
            x_neg: vec![grass],
            y_pos: vec![grass],
            y_neg: vec![grass],
            z_pos: vec![layer1_up, grass_up],
            z_neg: vec![layer1_down],
        })
        .with_weight(5.);
    sprite_index.push(Some(TileKind::GreenGrass.sprite_index()));

    // Grass Side, Corner and Inside

    let grass_side = SocketsCartesian3D::Simple {
        x_pos: void_and_grass,
        x_neg: grass_and_void,
        y_pos: void,
        y_neg: grass,
        z_pos: layer1_up,
        z_neg: layer1_down,
    }
    .to_template();

    let grass_cor = SocketsCartesian3D::Simple {
        x_pos: void_and_grass,
        x_neg: void,
        y_pos: void,
        y_neg: grass_and_void,
        z_pos: layer1_up,
        z_neg: layer1_down,
    }
    .to_template();

    let grass_in = SocketsCartesian3D::Simple {
        x_pos: grass_and_void,
        x_neg: grass,
        y_pos: grass,
        y_neg: void_and_grass,
        z_pos: layer1_up,
        z_neg: layer1_down,
    }
    .to_template();

    // Side
    models.create(grass_side.clone());
    sprite_index.push(Some(TileKind::GreenGrassSideT.sprite_index()));
    models.create(grass_side.clone().rotated(ModelRotation::Rot90, UP_AXIS));
    sprite_index.push(Some(TileKind::GreenGrassSideL.sprite_index()));
    models.create(grass_side.clone().rotated(ModelRotation::Rot180, UP_AXIS));
    sprite_index.push(Some(TileKind::GreenGrassSideB.sprite_index()));
    models.create(grass_side.clone().rotated(ModelRotation::Rot270, UP_AXIS));
    sprite_index.push(Some(TileKind::GreenGrassSideR.sprite_index()));

    // Corner
    models.create(grass_cor.clone());
    sprite_index.push(Some(TileKind::GreenGrassCornerTL.sprite_index()));
    models.create(grass_cor.clone().rotated(ModelRotation::Rot90, UP_AXIS));
    sprite_index.push(Some(TileKind::GreenGrassCornerBL.sprite_index()));
    models.create(grass_cor.clone().rotated(ModelRotation::Rot180, UP_AXIS));
    sprite_index.push(Some(TileKind::GreenGrassCornerBR.sprite_index()));
    models.create(grass_cor.clone().rotated(ModelRotation::Rot270, UP_AXIS));
    sprite_index.push(Some(TileKind::GreenGrassCornerTR.sprite_index()));

    // Inside
    models.create(grass_in.clone());
    sprite_index.push(Some(TileKind::GreenGrassInsideTL.sprite_index()));
    models.create(grass_in.clone().rotated(ModelRotation::Rot90, UP_AXIS));
    sprite_index.push(Some(TileKind::GreenGrassInsideBL.sprite_index()));
    models.create(grass_in.clone().rotated(ModelRotation::Rot180, UP_AXIS));
    sprite_index.push(Some(TileKind::GreenGrassInsideBR.sprite_index()));
    models.create(grass_in.clone().rotated(ModelRotation::Rot270, UP_AXIS));
    sprite_index.push(Some(TileKind::GreenGrassInsideTR.sprite_index()));

    (layer1_up, grass_up)
}