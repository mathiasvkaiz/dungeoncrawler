mod material;
mod planet;
mod scene;
mod voxel;

use bevy::prelude::*;
use planet::PlanetPlugin;
use scene::ScenePlugin;

/// Program entry point: configure the learning app, then start Bevy's runner.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((ScenePlugin, PlanetPlugin))
        .run();
}
