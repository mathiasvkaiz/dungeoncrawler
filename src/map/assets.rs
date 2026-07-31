use bevy::prelude::*;

use super::config::{ATLAS_COLUMNS, ATLAS_ROWS, TILE_PIXEL_SIZE};

#[derive(Resource)]
pub struct TilemapImage(pub Handle<Image>);

#[derive(Resource)]
pub struct TilemapLayout(pub Handle<TextureAtlasLayout>);

pub fn load_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image: Handle<Image> = asset_server.load("tilemap.png");

    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_PIXEL_SIZE),
        ATLAS_COLUMNS,
        ATLAS_ROWS,
        None,
        None,
    );
    let layout_handle = atlas_layout.add(layout);

    commands.insert_resource(TilemapImage(image));
    commands.insert_resource(TilemapLayout(layout_handle));
}