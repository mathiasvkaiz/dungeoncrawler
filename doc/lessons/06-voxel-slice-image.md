# 06 — Display a voxel cross-section

Status: complete for this increment (2026-09-21).
Prerequisite: lesson 05 palette restored to `Some([140, 95, 55, 255])` for soil.
Target: `learn/bevy-ecs`, `examples/learning/voxel.rs`, `planet.rs`, and `scene.rs`.
Version context: Bevy 0.19.0, Rust edition 2024, Rust/Cargo 1.97.0.

## Outcome

Replace the rotating square with a stationary, enlarged cross-section of the
sphere: gray rock inside brown soil, with transparent air outside. This first
image step toward A3 teaches voxel-to-pixel mapping and asset ownership. A slice
is a diagnostic view of the interior; the eventual globe needs surface visibility,
view direction, and lighting in later increments.

## Apply it yourself

### 1. Add a size reader in `examples/learning/voxel.rs`

Inside `impl VoxelGrid`, immediately before `fn index` and its doc comment, add:

```rust
    /// Called by readers as needed; return copied dimensions without exposing cells.
    pub fn size(&self) -> [usize; 3] {
        self.size
    }
```

### 2. Expose read access in `examples/learning/planet.rs`

Replace only the `PlanetVoxels` declaration (its comment, derive, and struct).
Add the accessor impl shown below. Keep `pub struct PlanetPlugin;` and the existing
`impl Plugin for PlanetPlugin` block: it still registers generation and reporting.

```rust
/// The ECS world owns this resource; it owns the plain Rust voxel grid.
#[derive(Resource)]
pub(crate) struct PlanetVoxels {
    grid: VoxelGrid,
}

impl PlanetVoxels {
    /// Called by presentation readers as needed; borrow the authoritative grid.
    pub(crate) fn grid(&self) -> &VoxelGrid {
        &self.grid
    }
}
```

The existing plugin implementation must remain alongside the new accessor impl:

```rust
impl Plugin for PlanetPlugin {
    /// Bevy calls this when main adds PlanetPlugin, before run().
    /// Register generation followed by its reader; neither runs here.
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (generate_planet, report_planet).chain());
    }
}
```

### 3. Replace all of `examples/learning/scene.rs`

```rust
use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::material::rgba;
use crate::planet::PlanetVoxels;
use crate::voxel::VoxelGrid;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    /// Bevy calls this during add_plugins; register a reader after Startup generation.
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
    let [width, height, depth] = grid.size();
    let image = slice_image(grid, depth / 2);
    let handle = images.add(image);

    commands.spawn(Camera2d);
    commands.spawn(Sprite {
        // Enlarge each source pixel to 40 world units without changing the image.
        custom_size: Some(Vec2::new(width as f32, height as f32) * 40.0),
        ..Sprite::from_image(handle)
    });
}

/// Called by setup_scene or tests; copy one XY layer into an sRGB image.
/// Walkthrough: allocate a clear image, visit its rows and columns, look up
/// each voxel's material color, and copy four bytes into the matching pixel.
/// Reverse Y so larger voxel Y appears higher on screen. Return the image.
/// Panics for an invalid layer, unknown material, or unsupported image dimensions.
fn slice_image(grid: &VoxelGrid, z: usize) -> Image {
    let [width, height, depth] = grid.size();
    assert!(z < depth, "slice outside grid");
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
            let material = grid.get([x as usize, y, z]).expect("in-bounds voxel");
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

    /// Cargo checks orientation, non-square dimensions, layer selection, and air.
    #[test]
    fn slice_preserves_positions_and_ignores_other_layers() {
        let mut grid = VoxelGrid::new([3, 2, 2]);
        assert!(grid.set([0, 1, 1], ROCK));
        assert!(grid.set([2, 0, 1], SOIL));
        assert!(grid.set([1, 1, 0], SOIL));
        let image = slice_image(&grid, 1);
        assert_eq!(image.texture_descriptor.size.width, 3);
        assert_eq!(image.texture_descriptor.size.height, 2);
        assert_eq!(image.pixel_bytes(UVec3::new(0, 0, 0)).unwrap(), rgba(ROCK).unwrap());
        assert_eq!(image.pixel_bytes(UVec3::new(2, 1, 0)).unwrap(), rgba(SOIL).unwrap());
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
        let sprite = sprites.single(app.world()).expect("one slice sprite");
        let images = app.world().resource::<Assets<Image>>();
        let image = images.get(&sprite.image).expect("sprite references stored image");
        assert_eq!(sprite.custom_size, Some(Vec2::splat(360.0)));
        assert_eq!(image.pixel_bytes(UVec3::new(4, 4, 0)).unwrap(), rgba(ROCK).unwrap());
        assert_eq!(image.pixel_bytes(UVec3::new(7, 4, 0)).unwrap(), rgba(SOIL).unwrap());
        assert_eq!(image.pixel_bytes(UVec3::new(8, 4, 0)).unwrap(), &[0, 0, 0, 0]);
    }
}
```

Keep `main.rs`, `material.rs`, and all other code unchanged.

## Verify

From the repository root:

```sh
cargo test --offline --locked --example learning
cargo run --offline --locked --example learning
```

Expect eleven passing tests and the same startup palette log. The new test uses an
asymmetric fixture because a symmetric sphere would hide an upside-down image.
It checks that another Z layer cannot leak into the selected slice. A second test
runs both plugins without a window and checks the sprite-to-image link and the
generated center slice. It initializes image storage explicitly because it omits
DefaultPlugins.

The window should show a stationary pixelated disk with a gray 3-by-3 core and
brown surrounding cells. The image is 9-by-9 pixels displayed at 360-by-360 world
units; transparent margins expose the background. At z=4, the occupied rows are:

```text
.........
....S....
..SSSSS..
..SRRRS..
.SSRRRSS.
..SRRRS..
..SSSSS..
....S....
.........
```

Here R is rock, S is soil, and dots are transparent air. This diagram describes
voxel materials; actual screen colors can be affected by the rendering pipeline.
The old square and its Spin system are intentionally removed. Close the window
to exit. Missing `PlanetVoxels` usually means the scene was registered in Startup
instead of PostStartup, or PlanetPlugin was omitted. Blurred edges suggest the
nearest sampler was missed. An opaque square suggests air's alpha is incorrect.

Tutor verification (2026-09-20): applied the exact three Rust blocks to a temporary
copy of the exercise in `/private/tmp/lesson06-txjp2ivs`, with copied manifest and
lockfile. Offline locked Cargo test for example learning compiled and passed all
11 tests using the shared target cache. Consulted the installed Bevy 0.19.0
cpu_draw example, image/sprite APIs, and main schedule source. No learner source
edits or GUI run; actual implementation and visual acceptance remain user checks.

## Design and concepts

> ℹ️ **Bevy — Registration order is not execution order**

`main` adds ScenePlugin and PlanetPlugin. Their `build` methods register systems;
they do not create the image. Bevy runs Startup generation and reporting, applies
queued commands under its default schedule settings, then runs PostStartup scene
setup once. The separate phase makes the dependency clear across plugins without
exposing the generator system just to order the scene after it.

```text
Startup:     generate grid → insert PlanetVoxels → report palette
PostStartup: borrow grid → build Image → add asset → queue camera and sprite
Frames:      Bevy renders the sprite using that image
```

Bevy supplies system parameters from the world: `Res<PlanetVoxels>` grants shared
access, `ResMut<Assets<Image>>` grants mutable access to image storage, and
`Commands` queues entity changes. DefaultPlugins provides image assets and rendering
in the actual app. The new test calls the helper directly, so it needs no window
or Bevy runner. It does not verify GPU output.

> ℹ️ **Ownership — Authoritative voxels and derived pixels**

`PlanetVoxels` still owns terrain. `grid()` lends a shared reference, with no clone
of the grid and no permission to mutate it. `pub(crate)` exposes the type and reader
within this executable crate; its field stays private to the planet module.
`size()` returns a copy of three integers, leaving cell storage private.

The new image is derived data: changing its pixels would not change terrain.
`images.add(image)` moves the image into Bevy's asset storage and returns a strong
`Handle<Image>`. The sprite owns that handle, keeping the asset referenced. A handle
identifies an asset; it does not embed a second image in the entity. Bevy uploads
image data for GPU display. CPU rendering here means CPU calculation of pixels;
showing the sprite still uses the GPU.

`MAIN_WORLD | RENDER_WORLD` combines flags to keep the image available to both app
code and rendering. Retaining CPU data is useful for later image updates. This
lesson creates the image once; later voxel or palette changes at runtime would
leave it stale until explicitly rebuilt. Add invalidation when runtime edits arrive.

> ℹ️ **Coordinates — A layer is not a surface projection**

For this 9-cubed grid, `depth / 2` selects z=4. Each output pixel reads exactly one
voxel `[x, y, 4]`. It never searches through depth for the nearest occupied voxel,
so rock is visible inside the cut. A future surface renderer will require that
visibility calculation. This slice is a temporary diagnostic, not the final globe.

Image row zero is at the top. Our voxel convention treats larger Y as up, so
`height - 1 - row` maps image row zero to voxel y=8 and image row eight to y=0.
The image's Z coordinate is always zero: it is one 2D image layer, independent of
the selected voxel Z layer. `Rgba8UnormSrgb` matches the palette's four encoded
bytes, and nearest sampling keeps source pixels crisp when enlarged.

> ℹ️ **Rust — Mutation, borrowing, and moves in this code**

`let mut image` lets `pixel_bytes_mut(&mut self, ...)` borrow its pixel data
mutably. That method returns a mutable byte slice; `copy_from_slice(&color)` copies
four bytes from a shared reference to the color array. The temporary mutable
borrow ends after the statement, so the next pixel can be borrowed. Returning
`image` moves it to the caller; adding it to assets moves it again.

`mut images: ResMut<Assets<Image>>` has two roles: `ResMut` requests exclusive
resource access from Bevy, while the mutable binding enables calling `add` through
that wrapper. `grid: &VoxelGrid` is only a shared borrow. `u32::try_from` checks
conversion to Bevy's image dimension type; `expect` supplies a diagnostic if it
fails. The `..Sprite::from_image(handle)` syntax fills the remaining struct fields
from the constructed sprite while overriding `custom_size`.

> ℹ️ **Tradeoffs — One image rather than entities for cells**

This view uses one image and one sprite for the whole slice. One entity per cell
would add unnecessary ECS and draw organization for this task. A separate pure
byte-buffer module could decouple pixel calculation from Bevy further; this small
helper keeps the first asset lesson focused and remains directly testable. Extract
that boundary when the renderer has independent consumers or more algorithms.

Panics are deliberate diagnostics for this generated, tiny fixture. Asset size
conversion does not enforce GPU limits or memory budgets; this is not an arbitrary
large-volume viewer. Validate external content and use recoverable errors when
loading user data. No performance claim or production readiness is implied.

## Experiment and discuss

Change only `slice_image(grid, depth / 2)` to `slice_image(grid, 1)` in setup_scene.
Predict the visible materials and shape before running. Does changing the displayed
slice change the startup palette log? Explain which data each reader uses.

Restore `depth / 2`, save, and run again. Share both observations or request
**Review my work**. Keep the restored version for the next increment.

## Checkpoint

- [x] Lesson prepared and isolated validation recorded.
- [x] User implemented the code; reviewed with no correctness findings.
- [x] All 11 actual-exercise tests passed (2026-09-21).
- [x] User confirmed the cross-section displays correctly.
- [x] User reported the z=1 experiment; discussed the single soil cell.
- [x] Center slice `depth / 2` verified in final source.

Decision: display a one-time cross-section through an Image asset, keeping terrain
ownership in PlanetVoxels. PostStartup separates presentation setup from generation.
Full surface rendering, rotation, lighting, and geography remain later work.

Review (2026-09-21): PlanetPlugin registration is restored; scene executable code
matches this lesson. All 11 tests passed on the actual exercise. Center slice
`depth / 2` was present. At review, visual/experiment observations were pending
and trailing whitespace at voxel.rs:26 was left for user cleanup.

Final closeout (2026-09-21): user confirmed the correct cross-section and reported
one central cell during the z=1 experiment. Clarified that [4,4,1] is distance 3
from the sphere center, so it is soil on the outer surface, not the rock core.
Final source uses `depth / 2`. User explicitly authorized whitespace cleanup;
removed trailing whitespace from voxel.rs. Executable code is unchanged from the
11-test passing review, so those results are reused. Visual evidence is the user's
report; no assistant GUI validation. Lesson complete for this increment.
