use bevy::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    /// Bevy calls this when main adds ScenePlugin, before run().
    /// Register the scene's systems; do not execute them here.
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene)
            .add_systems(Update, spin_squares);
    }
}

#[derive(Component)]
/// Per-entity rotation speed, read by the rotation system.
struct Spin {
    radians_per_second: f32,
}

/// Bevy runs this once during Startup, supplying Commands to queue world changes.
/// Create the camera and the square that the update system will rotate.
fn setup_scene(mut commands: Commands) {
    // Spawns are queued; Bevy applies them before our Update system runs.
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::splat(120.0)),
        Transform::default(),
        Spin {
            radians_per_second: 1.0,
        },
    ));
}

/// Bevy runs this during Update and supplies time and component access.
/// Rotate entities with both Spin and Transform; no matches means no iterations.
fn spin_squares(time: Res<Time>, mut squares: Query<(&Spin, &mut Transform)>) {
    for (spin, mut transform) in &mut squares {
        // Convert radians per second into rotation for this frame.
        transform.rotate_z(spin.radians_per_second * time.delta_secs());
    }
}
