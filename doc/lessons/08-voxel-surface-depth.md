# 08 — Keep the visible surface depth

Status: complete for this increment (2026-09-21).
Prerequisite: lesson 07 complete, forward scan restored at `f15c980`.
Target: `learn/bevy-ecs`, `examples/learning/scene.rs` only.
Version context: Bevy 0.19.0, Rust edition 2024, Rust/Cargo 1.97.0.

## Outcome

Display the same stationary pixelated outline in grayscale: the nearer center is
brighter than the farther edge. Keep each hit's material and depth together so
presentation can choose either material color or a diagnostic depth color.

## Apply it yourself

Replace all of `examples/learning/scene.rs` with this file. Existing grid,
generator, palette, module registration, and dependencies are the prerequisites.

```rust
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
    fn build(&self, app: &mut App) {
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
#[derive(Clone, Copy)]
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
    let span = (depth - 1).max(1) as f32;
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
```

## Verify

From the repository root:

```sh
cargo test --offline --locked --example learning
cargo run --offline --locked --example learning
```

Expect 12 passing tests. The material test still checks occlusion. The new test
checks that material and Z belong to the same first hit, that depth colors preserve
Y orientation and transparency, and that a single depth layer avoids division by
zero. The windowless integration test checks the generated sphere's actual image.

The window should show the same coarse outline and size, now gray with a brighter
center and darker outer cells. Outside remains transparent. This is a depth
diagnostic; there is no light source. The startup palette log remains unchanged.
A uniformly bright disk suggests the hit's Z was lost. An upside-down asymmetric
fixture suggests the Y mapping changed; sphere symmetry can hide that mistake.

Tutor verification (2026-09-21): extracted this exact Rust block into
`/private/tmp/lesson08-84z9yvhs`, using copied manifest/lockfile and other exercise
modules with the shared target cache. Offline locked example tests passed all 12.
Consulted installed Bevy 0.19.0 image and schedule sources. No learner source edits,
actual lesson 08 implementation review, or assistant GUI validation.

## Design and concepts

> ℹ️ **Depth — One hit, two facts**

For the center column, the scan returns `Some(SurfaceHit { material: SOIL, z: 1 })`.
For the right tip at x=7, y=4, the first hit is soil at z=4. Previously both
became brown; now their different depths become visible.

```text
9 layers:       z=0      z=1             z=4              z=8
Grayscale byte: 255      231             160               64
                near / bright                       far / dark
```

We divide Z by the last valid index (`depth - 1`) to get a fraction from 0 to 1.
The formula maps that fraction onto display bytes from 255 down to 64. A depth-1
grid has only z=0; using a denominator of 1 gives white without dividing by zero.
These values describe voxel-layer position, not distance from Bevy's camera.
They are stored as sRGB display bytes; they are not linear light intensities.

> ℹ️ **Bevy — The ECS boundary stays at the planet and image**

`SurfaceHit` is an ordinary Rust value used for one pixel, not a component or
resource. The planet resource owns the authoritative grid. The image asset owns
derived RGBA bytes, and the sprite holds its handle. We do not retain a separate
depth buffer: the hit exists just long enough to choose the pixel color.

`ScenePlugin::build` registers setup during `add_plugins`. Bevy later runs setup
once in PostStartup, supplying shared `Res<PlanetVoxels>`, exclusive
`ResMut<Assets<Image>>`, and `Commands`. The helper functions run through ordinary
Rust calls inside setup; Bevy does not schedule them independently.

```text
Startup      generate grid → insert resource → report samples
PostStartup  setup → surface_image → first_hit → choose color → store image
Frames       display the sprite using its stored image handle
```

> ℹ️ **Rust — A result with named fields and an explicit choice**

`Option<SurfaceHit>` distinguishes a hit from an empty column. `Some(hit)` binds
the returned value; `hit.material` and `hit.z` access its fields. `None` produces
transparent pixels. `return` leaves `first_hit`, while the caller's pixel loops
continue. The struct initializer `{ material, z }` uses matching variable names
as shorthand for `{ material: material, z: z }`.

The enum makes the two display choices explicit. `match view` handles both choices;
`Clone, Copy` allow this small fieldless enum to be copied. `Debug, PartialEq, Eq`
on the hit let tests print and compare its fields; these are ordinary Rust derives.

`grid: &VoxelGrid` only borrows terrain for reading. `let mut image` allows mutable
pixel access. `mut images: ResMut<Assets<Image>>` combines a mutable local binding
with Bevy's exclusive access to the asset resource. None of these changes terrain.

> ℹ️ **Tradeoff — Diagnostic color is a presentation choice**

Keeping depth beside material separates finding the surface from choosing its
appearance. Writing only grayscale during the scan would be shorter but discard
the material needed by the existing view. A persistent hit buffer would enable
recoloring without rescanning, at the cost of storage and invalidation work; defer
it until runtime changes require it.

The renderer still assumes every non-air voxel is opaque. Depth view works for
any non-air ID; material view requires a palette entry. The view is fixed along
+Z. This once-only image does not respond to runtime grid or view changes.
Worst-case traversal still reads width × height × depth cells; no speed claim or
measurement is part of this increment. Arbitrary rotation and lighting remain later
work. Keep the small grid: large external volumes need explicit size limits.

## Experiment and discuss

In `setup_scene`, change only `SurfaceView::Depth` to `SurfaceView::Material`.
Predict the center and edge colors, then restart the app and compare the result.
Explain why one view reveals shape that the other hides even though both call
`first_hit`. The depth-specific integration test intentionally expects the default
view, so changing setup also changes that test's result.

Restore `SurfaceView::Depth`, rerun the tests, and share what you observed or ask
for **Review my work**. No runtime toggle is needed for this experiment.

## Checkpoint

- [x] Lesson prepared and isolated validation recorded.
- [x] User applied the replacement scene.
- [x] Actual tests passed and user visual result recorded.
- [x] View comparison discussed and depth view restored.

Decision: retain a local material/Z hit and choose its display color separately.
Keep existing resource and asset ownership; revisit retained hit data when runtime
recoloring or picking needs it. Stop here for implementation and discussion.

User observations (2026-09-21): Depth shows differently gray-colored cross pixels;
Material shows uniform brown again. Depth varies between hits while the visible
soil material stays the same. Source confirms Depth restored. Review found no correctness issues, and all 12
actual exercise tests passed. No assistant GUI validation.

Close review (2026-09-21): checked first-hit material/Z coupling, normalization,
empty columns, Y mapping, PostStartup ordering, and asset/resource ownership.
The user's enum omits Clone/Copy; this is valid because matching these fieldless
variants does not move any data. No correctness findings. User authorized comment
typo/doc-comment marker corrections and removal of two trailing-whitespace lines;
executable code is unchanged, so the 12 passing actual tests remain applicable.
Lesson complete for this increment; no pending exercise.
