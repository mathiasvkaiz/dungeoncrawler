mod components;

use crate::components::*;
use bevy::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
    .add_systems(Startup, setup)
    .run();
}

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
  commands.spawn(Camera2d);

  // Load the sprite sheet using `AssetServer`
  let texture: Handle<Image> = asset_server.load("gabe-idle-run.png");

  // The sprite sheet has 7 sprites arranged in a row, they are all 24px X 24px
  let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
  let texture_atlas_layout = texture_atlas_layouts.add(layout);

  let animation_config = AnimationConfig::new(1, 6, 10);

  commands.spawn((
    Sprite {
      image: texture.clone(),
      texture_atlas: Some(TextureAtlas {
        layout: texture_atlas_layout.clone(),
        index: animation_config.first_sprite_index,
      }),
      ..default()
    },
    Transform::from_scale(Vec3::splat(6.0))
      .with_translation(Vec3::new(-70.0, 0.0, 0.0)),
    MainSprite,
    animation_config,
  ));
}