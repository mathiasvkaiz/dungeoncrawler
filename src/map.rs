mod assets;
pub mod config;
mod generation;
mod rules;
mod tiles;

use bevy::prelude::*;

use assets::load_tilemap;
use generation::generate_map;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_tilemap, generate_map).chain());
    }
}