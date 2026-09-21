# 05 — Map material IDs to colors

Status: complete for this increment; restoration verified in source and tests.
Prerequisite: lesson 04 complete; radius 3.0 and soil thickness 1.0 restored.
Target: `learn/bevy-ecs`, `examples/learning/`.
Version context: Bevy 0.19.0, Rust edition 2024, Rust/Cargo 1.97.0.

## Outcome

Give each known material a display color without changing the voxel grid.
Startup prints sampled colors. The rotating square remains the visual placeholder;
this small A2 increment prepares data for a later CPU renderer.

## Apply it yourself

### 1. Create `examples/learning/material.rs`

```rust
// Shared IDs: the generator and presentation must agree on their meaning.
pub const AIR: u8 = 0;
pub const ROCK: u8 = 1;
pub const SOIL: u8 = 2;

/// Called by readers as needed; map an ID to copied RGBA bytes.
/// RGB channels are sRGB encoded; alpha is 0 for clear and 255 for opaque.
/// Unknown IDs return None, rather than silently looking like air.
pub fn rgba(material: u8) -> Option<[u8; 4]> {
    match material {
        AIR => Some([0, 0, 0, 0]),
        ROCK => Some([110, 115, 125, 255]),
        SOIL => Some([140, 95, 55, 255]),
        _ => None,
    }
}
```

### 2. Replace all of `examples/learning/main.rs`

```rust
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
```

### 3. Make three focused edits in `examples/learning/planet.rs`

Replace the imports and three material constants at the top, stopping before
`pub struct PlanetPlugin;`, with:

```rust
use bevy::prelude::*;

use crate::material::{AIR, ROCK, SOIL, rgba};
use crate::voxel::VoxelGrid;
```

Replace the entire `report_planet` function and its doc comments with:

```rust
/// Bevy runs this once after generation and its queued insertion have completed.
/// Bevy supplies shared resource access; ordinary Rust calls resolve the colors.
fn report_planet(planet: Res<PlanetVoxels>) {
    info!(
        "Palette: center={:?}, surface={:?}, air={:?}",
        planet.grid.get([4, 4, 4]).and_then(rgba),
        planet.grid.get([7, 4, 4]).and_then(rgba),
        planet.grid.get([8, 4, 4]).and_then(rgba),
    );
}
```

Inside the existing `#[cfg(test)] mod tests`, immediately after `use super::*;`,
add this test. Keep all existing tests and the rest of the file.

```rust
    /// Cargo checks that generated IDs resolve and missing data stays distinct.
    #[test]
    fn generated_materials_have_valid_palette_entries() {
        let grid = solid_sphere([9, 9, 9], 3.0);
        for z in 0..9 {
            for y in 0..9 {
                for x in 0..9 {
                    let id = grid.get([x, y, z]).expect("in-bounds cell");
                    let color = rgba(id).expect("generated material has a color");
                    assert_eq!(color[3], if id == AIR { 0 } else { 255 });
                }
            }
        }
        assert_eq!(grid.get([8, 4, 4]).and_then(rgba), Some([0, 0, 0, 0]));
        assert_eq!(grid.get([9, 4, 4]).and_then(rgba), None);
        assert_eq!(rgba(255), None);
    }
```

## Verify

From the repository root:

```sh
cargo test --offline --locked --example learning
cargo run --offline --locked --example learning
```

Expect nine passing tests. The new test checks compatibility between generated
IDs and the palette across the whole small sphere, transparency for air, opacity
for occupied cells, invalid coordinates, and an unknown ID. It deliberately does
not freeze the artistic RGB choices.

Expect this message once at startup:

```text
Palette: center=Some([110, 115, 125, 255]), surface=Some([140, 95, 55, 255]), air=Some([0, 0, 0, 0])
```

The square should still rotate with its original blue color. It has its own sprite
color and does not read this palette. Close the window to exit. An unresolved
`crate::material` usually means the module declaration or new file is missing.
Duplicate definitions of `AIR`, `ROCK`, or `SOIL` mean the old constants remain.
A palette `None` for an in-bounds sample means its ID has no palette entry.

Tutor verification (2026-09-20): applied these exact Rust blocks to a temporary
copy of the exercise at `/private/tmp/lesson05-3pskxtvx`, with copied manifest and
lockfile. Offline locked Cargo test for example learning compiled and passed all
nine tests using the shared target cache. Consulted installed Bevy 0.19.0 app and
schedule sources. At preparation time, no learner source/dependency edits or actual lesson 05 review,
startup-log observation, or GUI validation performed. These checks establish
lesson preparation only.

## Design and concepts

> ℹ️ **Data — Identity belongs to terrain; color belongs to presentation**

A cell still stores one byte. `ROCK` identifies what occupies it; `[110, 115, 125,
255]` describes how we intend to display it. Recoloring rock does not require
regenerating the sphere or changing its material IDs. The new shared module
prevents the generator and palette from independently assigning different meanings
to the same number. `VoxelGrid` remains general byte storage with zero as air.

RGBA means red, green, blue, alpha, each from 0 to 255. We choose sRGB-encoded RGB
bytes for future display; a renderer will need a matching image format. These
are base colors, with no lighting calculation. Transparency is a display value;
future voxel traversal should use the AIR identity to skip empty cells.

> ℹ️ **Bevy — A helper function does not need a system or resource**

`main` adds `PlanetPlugin`; Bevy calls its `build` method immediately to register
Startup systems. On startup, Bevy runs `generate_planet`, applies its queued
resource insertion before the chained `report_planet`, and supplies the latter
with shared `Res<PlanetVoxels>` access. The reader calls `rgba` as ordinary Rust.
The lookup has no schedule and Bevy does not call it independently.

```text
Startup: generate grid → insert resource → read ID → look up color → log
Update:  rotate the existing square each frame
```

The world still owns one `PlanetVoxels` resource containing the grid. The fixed
palette is code, so it needs no ECS entity or resource. If runtime palette editing
becomes a requirement, a palette resource could hold mutable entries and readers
could request `Res<MaterialPalette>`. That introduces lifecycle and access concerns
we do not need for three fixed entries.

> ℹ️ **Rust — Modules, matching, and optional results**

`mod material;` includes the sibling file as a module. `pub` makes its constants
and function accessible from `planet`; `use` brings their names into scope.
`match` compares the byte to the named constants, and `_` catches every other
value. Each arm returns the same type, `Option<[u8; 4]>`: either four bytes in
`Some`, or `None` for an unknown material. The small array is returned by value.

Walk through `get(...).and_then(rgba)` with the surface sample: `get` returns
`Some(2)`, `and_then` passes `2` to `rgba`, and the result is
`Some([140, 95, 55, 255])`. For an invalid coordinate, `get` returns `None` and
skips the lookup. Passing `rgba` without parentheses passes the function itself.
`map(rgba)` would instead produce nested options because `rgba` already returns
an `Option`. This diagnostic deliberately combines invalid coordinates and unknown
IDs into one `None`; retain the coordinate and ID separately when detailed error
reporting is needed. Valid air is still `Some([0, 0, 0, 0])`.

Neither the palette nor reader needs `mut`: both only read. In generation,
`let mut grid` allows `set` to take `&mut self`; binding mutability enables that
mutable borrow. `Res<PlanetVoxels>` only gives shared access. Adding `mut` to its
binding would not grant mutable access to the resource; that requires `ResMut`.

> ℹ️ **Design — Keep the palette small and explicit**

A `match` is enough for three entries and handles unknown IDs explicitly. A table
indexed by material ID is another option, but needs bounds handling and a policy
for unused entries. Data-driven palettes become useful when content is loaded or
edited independently of the program. Neither approach has been benchmarked here.
Unknown IDs do not silently become air; that would hide missing content. A later
renderer can choose to report an error or display a diagnostic color.

## Experiment and discuss

Change only SOIL's palette entry to `Some([60, 160, 70, 255])`. Before running,
predict which startup samples change, whether any grid IDs change, and whether
the rotating square changes color. Run the tests and app, then compare.

Restore the original soil color and rerun. Bring your observations or request
**Review my work**. Rendering the planet remains a later increment.

## Checkpoint

- [x] Lesson prepared and isolated validation recorded.
- [x] User applied the code; reviewed with no correctness findings.
- [x] Actual exercise tests, startup log, and continued rotation verified (user app observations).
- [x] Palette experiment reported; only the surface sample changed.
- [x] Original soil color restored; source verified and nine tests passed.

Decision: share material IDs and a pure fixed palette in one module; retain byte
storage and the existing ECS ownership. Revisit a palette resource when runtime
editing is needed. Review (2026-09-20): executable changes matched the lesson and all
nine actual-exercise tests passed. At close, user reported both expected logs and
continued rotation. Green SOIL [60,160,70,255] remains in source; all nine tests
passed again on that final version. No assistant GUI validation or source edits.
Subsequent resume: user corrected the restoration typo; at 9a37e6f source has
`Some([140, 95, 55, 255])` and all nine actual tests pass. User requested progression
and confirmed the correction. Earlier user logs establish baseline and experiment
observations; no new post-correction GUI observation was supplied or inferred.
Lesson 06 is now prepared separately.
