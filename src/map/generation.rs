use bevy::prelude::*;
use ghx_proc_gen::generator::{builder::GeneratorBuilder, RngMode};
use ghx_proc_gen::ghx_grid::cartesian::grid::CartesianGrid;

use super::assets::{TilemapImage, TilemapLayout};
use super::config::{MAP_LAYERS, MAP_TILES_X, MAP_TILES_Y, TILE_PIXEL_SIZE};
use super::rules::build_rules;

pub fn generate_map(
    mut commands: Commands,
    tilemap_image: Res<TilemapImage>,
    tilemap_layout: Res<TilemapLayout>,
) {
    let (rules, sprite_indices) = build_rules();

    let grid =
        CartesianGrid::new_cartesian_3d(MAP_TILES_X, MAP_TILES_Y, MAP_LAYERS, false, false, false);

    let world_seed = RngMode::RandomSeed;

    let mut generator = GeneratorBuilder::new()
        .with_rules(rules)
        .with_grid(grid)
        .with_rng(world_seed)
        .build()
        .expect("failed to build the generator");

    let (_gen_info, grid_data) = generator
        .generate_grid()
        .expect("WFC generation failed (contradiction)");

    let tile_size = TILE_PIXEL_SIZE as f32;
    let half_map_x = (MAP_TILES_X as f32 * tile_size) / 2.0;
    let half_map_y = (MAP_TILES_Y as f32 * tile_size) / 2.0;

    let map_parent = commands
        .spawn((
            Name::new("Map"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    let size_x = grid_data.grid().size_x();
    let size_y = grid_data.grid().size_y();
    let size_z = grid_data.grid().size_z();

    for z in 0..size_z {
        for y in 0..size_y {
            for x in 0..size_x {
                let model_instance = grid_data.get_3d(x, y, z);

                let Some(sprite_index) = sprite_indices[model_instance.model_index] else {
                    continue;
                };

                let world_x = x as f32 * tile_size - half_map_x + tile_size / 2.0;
                let world_y = y as f32 * tile_size - half_map_y + tile_size / 2.0;
                let world_z = z as f32;

                commands.entity(map_parent).with_children(|parent| {
                    parent.spawn((
                        Sprite::from_atlas_image(
                            tilemap_image.0.clone(),
                            TextureAtlas {
                                layout: tilemap_layout.0.clone(),
                                index: sprite_index,
                            },
                        ),
                        Transform::from_xyz(world_x, world_y, world_z),
                    ));
                });
            }
        }
    }
}