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
    let image = surface_image(grid, SurfaceView::Depth);
    let handle = images.add(image);

    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        // Enlarge each source pixel to 40 world units without changing the image.
        custom_size: Some(Vec2::new(width as f32, height as f32) * 40.0),
        ..Sprite::from_image(handle)
    });
}

/// A temporary scan result, owned locally by the renderer; not an ECS component.
#[derive(Debug, PartialEq, Eq)]
struct SurfaceHit {
    material: u8,
    z: usize,
}

/// Choose how to display the same visible surface.
enum SurfaceView {
    Material,
    Depth,
}

/// Called by surface_image or tests for a valid XY column; return its nearest solid.
/// The early return keeps material and depth from the same cell together.
fn first_hit(grid: &VoxelGrid, x: usize, y: usize) -> Option<SurfaceHit> {
    for z in 0..grid.size()[2] {
        let material = grid.get([x, y, z]).expect("in-bounds voxel");
        if material != AIR {
            return Some(SurfaceHit { material, z });
        }
    }
    None
}

/// Called by surface_image or tests; encode a valid hit's Z as opaque grayscale.
/// Low Z is bright; high Z is dark. A one-layer grid uses the nearest value.
fn depth_rgba(z: usize, depth: usize) -> [u8; 4] {
    assert!(z < depth, "depth sample out of bounds");
    let span = (depth -1).max(1) as f32;
    let fraction = z as f32 / span;
    // Keep the far plane visible; these are display bytes, not physical lighting.
    let value = (255.0 - 191.0 * fraction).round() as u8;
    [value, value, value, 255]
}

/// Called by setup_scene or tests; render fixed-axis hits as materials or depth.
/// Walkthrough: allocate a clear image, visit each XY column, find its first hit,
/// then select that hit's display color. Empty columns stay transparent.
/// Reverse Y so larger voxel Y appears at the top.
/// Panics for unsupported image dimensions or unknown visible IDs in material view.
fn surface_image(grid: &VoxelGrid, view: SurfaceView) -> Image {
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
            let color = match first_hit(grid, x as usize, y) {
                Some(hit) => match view {
                    SurfaceView::Material => rgba(hit.material)
                        .expect("material missing from palette"),
                    SurfaceView::Depth => depth_rgba(hit.z, depth),
                },
                None => [0, 0, 0, 0],
            };
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
        let image = surface_image(&grid, SurfaceView::Material);
        assert_eq!(image.texture_descriptor.size.width, 3);
        assert_eq!(image.texture_descriptor.size.height, 2);
        assert_eq!(image.pixel_bytes(UVec3::new(0, 0, 0)).unwrap(), rgba(SOIL).unwrap());
        assert_eq!(image.pixel_bytes(UVec3::new(2, 1, 0)).unwrap(), rgba(ROCK).unwrap());
        assert_eq!(image.pixel_bytes(UVec3::new(2, 0, 0)).unwrap(), rgba(SOIL).unwrap());
        assert_eq!(image.pixel_bytes(UVec3::new(1, 0, 0)).unwrap(), &[0, 0, 0, 0]);
    }

    /// Cargo checks coupled hit data, clear columns, depth colors and Y orientation.
    #[test]
    fn depth_view_preserves_first_hit_distance() {
        let mut grid = VoxelGrid::new([3, 2, 4]);
        assert!(grid.set([0, 1, 1], SOIL));
        assert!(grid.set([0, 1, 2], ROCK));
        assert!(grid.set([2, 0, 3], ROCK));
        assert!(grid.set([2, 1, 0], SOIL));
        assert_eq!(first_hit(&grid, 0, 1), Some(SurfaceHit { material: SOIL, z: 1 }));
        assert_eq!(first_hit(&grid, 1, 1), None);
        let image = surface_image(&grid, SurfaceView::Depth);
        assert_eq!(image.pixel_bytes(UVec3::new(0, 0, 0)).unwrap(), &[191, 191, 191, 255]);
        assert_eq!(image.pixel_bytes(UVec3::new(2, 1, 0)).unwrap(), &[64, 64, 64, 255]);
        assert_eq!(image.pixel_bytes(UVec3::new(2, 0, 0)).unwrap(), &[255, 255, 255, 255]);
        assert_eq!(image.pixel_bytes(UVec3::new(1, 0, 0)).unwrap(), &[0, 0, 0, 0]);

        let mut thin = VoxelGrid::new([1, 1, 1]);
        assert!(thin.set([0, 0, 0], ROCK));
        let image = surface_image(&thin, SurfaceView::Depth);
        assert_eq!(image.pixel_bytes(UVec3::ZERO).unwrap(), &[255, 255, 255, 255]);
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
        assert_eq!(image.pixel_bytes(UVec3::new(4, 4, 0)).unwrap(), [231, 231, 231, 255]);
        assert_eq!(image.pixel_bytes(UVec3::new(7, 4, 0)).unwrap(), [160, 160, 160, 255]);
        assert_eq!(image.pixel_bytes(UVec3::new(8, 4, 0)).unwrap(), &[0, 0, 0, 0]);
    }
}
