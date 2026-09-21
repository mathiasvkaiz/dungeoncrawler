# 07 — Find the visible voxel surface

Status: complete for this increment (2026-09-21).
Prerequisite: lesson 06 complete, with the center slice restored.
Target: `learn/bevy-ecs`, `examples/learning/scene.rs` only.
Version context: Bevy 0.19.0, Rust edition 2024, Rust/Cargo 1.97.0.

## Outcome

Display the sphere's front surface: a stationary brown pixelated disk with
transparent air around it. Soil now hides the rock core. This small A3 step adds
visibility along one fixed axis; view rotation and lighting come later.

## Apply it yourself

Replace all of `examples/learning/scene.rs` with this file. The grid, generator,
palette, main module, and dependencies already provide everything it needs.

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
/// if the whole column is empty. Reverse Y to keep larger voxel Y at the top.
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
```

## Verify

From the repository root:

```sh
cargo test --offline --locked --example learning
cargo run --offline --locked --example learning
```

Expect 11 passing tests. The two scene tests replace the previous slice tests.
The asymmetric fixture checks that nearer soil hides farther rock, a cell at the
last depth is still found, a cell at z=0 is included, empty columns stay clear,
and the image preserves dimensions and orientation. The windowless integration
test checks generation, image storage, and the sprite's handle together.

The window should show the same disk outline and size as lesson 06, now entirely
brown inside. It remains stationary and unlit. The startup palette log still
reports a gray rock center: that reader samples the original grid, not the image.
Gray in the middle suggests setup still calls the slice renderer. Seeing a farther
material in the fixture suggests the depth loop does not stop at its first hit.

Tutor verification (2026-09-21): extracted the exact Rust block into a temporary
exercise copy at `/private/tmp/lesson07-7pkns_vi`, with copied manifest/lockfile
and the shared target cache. `cargo test --offline --locked --example learning`
passed all 11 tests there. Checked installed Bevy 0.19.0 image and schedule sources.
No learner source edits, actual implementation review, or GUI validation occurred.

## Design and concepts

> ℹ️ **Visibility — Follow one column**

For the center pixel, x=4 and y=4 remain fixed while Z advances:

```text
view direction → increasing Z
z:       0 1 2 3 4 5 6 7 8
material . S S R R R S S .
           ↑ first hit: soil supplies this pixel
```

The slice read z=4 and showed rock. The surface view stops at z=1. Both read the
same terrain; only the visibility rule changes. Each pixel uses a parallel column,
so this is an orthographic view along +Z in voxel coordinates. Bevy's Camera2d
still sees a flat sprite; its transform does not define the voxel viewing axis.

All non-air IDs count as solid. Occupancy is decided before palette lookup; this
renderer assumes opaque solid materials. Glass or partial transparency would need
an explicit transparency rule and blending through multiple cells. Unknown visible
IDs still fail at the palette lookup instead of silently becoming air.

> ℹ️ **Bevy — One setup system, ordinary Rust work inside it**

`ScenePlugin::build` registers `setup_scene`; it does not execute the scan.
Bevy runs `setup_scene` once in PostStartup, after Startup generation and queued
resource insertion. It supplies shared `Res<PlanetVoxels>`, exclusive
`ResMut<Assets<Image>>`, and `Commands` for queued entity creation.
`surface_image` is an ordinary function called by setup or tests, not a system.

```text
Startup      generate grid → insert resource → report fixed samples
PostStartup  borrow grid → scan columns → add image asset → spawn sprite
Frames       Bevy displays the stored image
```

The resource still owns the authoritative grid. The image owns derived pixels in
`Assets<Image>`, and the sprite retains its strong asset handle. No voxel entities
are needed. Runtime terrain or palette edits would require an explicit image update;
this once-only setup does not detect them.

> ℹ️ **Rust — Local mutation does not mutate the terrain**

`let mut material = AIR` creates a new mutable byte binding for each pixel. Assigning
`candidate` copies a byte; it does not borrow or modify a cell. `grid: &VoxelGrid`
only permits shared access. `break` exits the innermost loop, so the next pixel
still gets processed. An empty column keeps its initial AIR value.

`let [width, height, _]` ignores the depth in setup; the renderer reads it itself.
`let mut image` allows borrowing the image mutably to write pixels. In contrast,
`mut images: ResMut<Assets<Image>>` combines Bevy's exclusive resource access with
a mutable binding that can call `add`. The grid remains shared throughout.

> ℹ️ **Tradeoff — A fixed axis keeps visibility small**

This scan uses the existing grid accessor and image pipeline. In the worst case it
reads width × height × depth cells; early hits shorten individual scans. No timing
or performance improvement has been measured. Arbitrary viewing angles need a
more general ray traversal; defer that until this visibility rule is understood.
The small fixture still uses panics for invalid palette data and unsupported sizes;
external volumes would need validation and memory/GPU limits.

## Experiment and discuss

In `surface_image`, change `for z in 0..depth` to
`for z in (0..depth).rev()`. Predict whether the sphere image will change, then run
it. Run the tests too: compare the result for the asymmetric fixture's top-left
pixel. Explain why that fixture can distinguish scan direction even if the sphere
cannot. `.rev()` visits the same depths in reverse order.

Restore `for z in 0..depth` and rerun the tests. Share the observations or request
**Review my work** before advancing.

## Checkpoint

- [x] Lesson prepared and isolated validation recorded.
- [x] User implemented the replacement scene; reviewed without correctness findings.
- [x] All 11 actual tests passed; user confirmed the outline and brown center.
- [x] Reverse-scan observations discussed and forward scan verified restored.

Decision: scan low Z to high Z, stop at the first non-air material, and retain the
existing image/resource ownership. Revisit traversal for arbitrary views and image
updates when runtime changes arrive. No rotation, lighting, or geography in this step.

Review (2026-09-21): actual exercise passes all 11 tests. User reports the same
cross-like outline as lesson 06 and unchanged appearance with reversed scanning,
while a test failed during the experiment. The asymmetric fixture selects rock
instead of soil when scanned backward; the sphere has soil on both sides. The
reported failure log was not supplied. Current source uses forward scanning.
User subsequently confirmed the visible center is brown. Lesson complete for this
increment; no pending exercise. Visual evidence is user-reported, with no assistant
GUI validation. Reused the passing tests; no source changes for this confirmation.
