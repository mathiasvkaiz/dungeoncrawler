use bevy::prelude::*;

mod map;
mod player;
use map::config::{WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() {
    let dimension = (WINDOW_WIDTH, WINDOW_HEIGHT);

    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("mmo game"),
                        resolution: dimension.into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup_camera)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(map::MapPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}