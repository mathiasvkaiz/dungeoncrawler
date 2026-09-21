use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::material::{AIR, rgba};
use crate::planet::PlanetVoxels;
use crate::voxel::VoxelGrid;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    /// Bevy calls this during add_plugins: register a reader after Startup generation.
    fn build(&self, app:&mut App) {
        app.add_systems(PostStartup, setup_scene);
    }
}

/// Bevy runs this once in PostStartup, after Startup's queued resource insertion.
/// It supplies the planet, mutable image assets, and a queue for entity creation.
fn setup_scene(
    mut commands: Commands,
    planet: Res<PlanetVoxels>,
    mut images: ResMut<Assets<Image>>,
) {
    let grid = planet.grid();
    let [width, height, _] = grid.size();
    let image = surface_image(grid);
    let handle = images.add(image);

    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        // Enlarge each source pixel to 40 world units without changing the image.
        custom_size: Some(Vec2::new(width as f32, height as f32) * 40.0),
        ..Sprite::from_image(handle)
    });
}

/// Called by setup_scene or tests; view the grid from low Z toward high Z.
/// Walkthrough: allocate a clear image, visit each pixel's XY column, then scan
/// its depth until the first non-air cell. Copy that material's color, or air
/// if the whole column is empty. Reverse Y to keep larger voxel > at the top.
/// Panics for an unknown visible material or unsupported image dimensions.
fn surface_image(grid: &VoxelGrid) -> Image {
    let [width, height, depth] = grid.size();
    let image_width = u32::try_from(width).expect("image width exceeds u32");
    let image_height = u32::try_from(height).expect("image height exceeds u32");
    let mut image = Image::new_fill(
        Extent3d {
            width: image_width,
            height: image_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest();

    for row in 0..image_height {
        let y = height - 1 - row as usize;
        for x in 0..image_width {
            let mut material = AIR;
            for z in 0..depth {
                let candidate = grid.get([x as usize, y, z]).expect("in-bounds voxel");
                if candidate != AIR {
                    material = candidate;
                    // Stop only the depth loop: nearer solid cells hide farther ones.
                    break;
                }
            }
            let color = rgba(material).expect("material missing from palette");
            image
                .pixel_bytes_mut(UVec3::new(x, row, 0))
                .expect("in-bounds RGBA pixel")
                .copy_from_slice(&color);
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::{ROCK, SOIL};

    /// Cargo checks near-hit occlusion, empty columns, depth endpoints, and Y mapping.
    #[test]
    fn surface_selects_first_solid_in_each_column() {
        let mut grid = VoxelGrid::new([3, 2, 4]);
        assert!(grid.set([0, 1, 1], SOIL));
        assert!(grid.set([0, 1, 2], ROCK));
        assert!(grid.set([2, 0, 3], ROCK));
        assert!(grid.set([2, 1, 0], SOIL));
        let image = surface_image(&grid);
        assert_eq!(image.texture_descriptor.size.width, 3);
        assert_eq!(image.texture_descriptor.size.height, 2);
        assert_eq!(image.pixel_bytes(UVec3::new(0, 0, 0)).unwrap(), rgba(SOIL).unwrap());
        assert_eq!(image.pixel_bytes(UVec3::new(2, 1, 0)).unwrap(), rgba(ROCK).unwrap());
        assert_eq!(image.pixel_bytes(UVec3::new(2, 0, 0)).unwrap(), rgba(SOIL).unwrap());
        assert_eq!(image.pixel_bytes(UVec3::new(1, 0, 0)).unwrap(), &[0, 0, 0, 0]);
    }

    /// Cargo runs both plugins without a window to check startup and asset wiring.
    #[test]
    fn scene_uses_generated_planet_image() {
        let mut app = App::new();
        app.init_resource::<Assets<Image>>()
            .add_plugins((ScenePlugin, crate::planet::PlanetPlugin));
        app.update();
        let mut sprites = app.world_mut().query::<&Sprite>();
        let sprite = sprites.single(app.world()).expect("one surface sprite");
        let images = app.world().resource::<Assets<Image>>();
        let image = images.get(&sprite.image).expect("sprite references stored image");
        assert_eq!(sprite.custom_size, Some(Vec2::splat(360.0)));
        assert_eq!(image.pixel_bytes(UVec3::new(4, 4, 0)).unwrap(), rgba(SOIL).unwrap());
        assert_eq!(image.pixel_bytes(UVec3::new(7, 4, 0)).unwrap(), rgba(SOIL).unwrap());
        assert_eq!(image.pixel_bytes(UVec3::new(8, 4, 0)).unwrap(), &[0, 0, 0, 0]);
    }
}
