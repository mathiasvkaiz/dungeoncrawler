# 04 — Assign materials inside the sphere

Status: complete and reviewed for this increment (2026-09-20).
Prerequisite: lesson 03 complete, radius 3.0 restored in planet.rs.
Target: `learn/bevy-ecs`, `examples/learning/planet.rs`.
Version context: Bevy 0.19.0, Rust edition 2024, Rust/Cargo 1.97.0.

## Outcome

Give the sphere a rock core and a soil layer one cell spacing thick. Startup
samples will distinguish air (0), rock (1), and soil (2). The rotating square
remains the visual placeholder; this increment assigns data, not colors.

## Apply it yourself

Replace **all of `examples/learning/planet.rs`** with this code. The other
exercise files and dependencies stay as they are.

```rust
use bevy::prelude::*;

use crate::voxel::VoxelGrid;

// IDs stored in the grid; colors will belong to a later presentation palette.
const AIR: u8 = 0;
const ROCK: u8 = 1;
const SOIL: u8 = 2;

pub struct PlanetPlugin;

/// The ECS world owns this resource; it owns the plain Rust voxel grid.
#[derive(Resource)]
struct PlanetVoxels {
    grid: VoxelGrid,
}

impl Plugin for PlanetPlugin {
    /// Bevy calls this when main adds PlanetPlugin, before run().
    /// Register generation followed by its reader; neither runs here.
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (generate_planet, report_planet).chain());
    }
}

/// Bevy runs this once in Startup, supplying a deferred command queue.
/// Build the data locally, then queue transfer of ownership to the world.
fn generate_planet(mut commands: Commands) {
    let grid = solid_sphere([9, 9, 9], 3.0);
    commands.insert_resource(PlanetVoxels { grid });
}

/// Called by generate_planet or tests; build a solid sphere with a soil layer.
/// Radius and the fixed layer thickness are measured in cell spacings.
/// Walkthrough: start with air, find the center, then visit each cell.
/// Leave cells outside the sphere as air. Inside, choose rock for the core
/// and soil for the outer layer. Return the finished grid to the caller.
/// For radius 3, distances below 2 are rock; distances 2 through 3 are soil.
/// Oversized spheres are clipped. Panics for invalid dimensions or radius.
fn solid_sphere(size: [usize; 3], radius: f32) -> VoxelGrid {
    assert!(radius.is_finite() && radius >= 0.0, "invalid radius");
    let mut grid = VoxelGrid::new(size);
    let center = size.map(|axis| (axis as f32 - 1.0) * 0.5);
    let radius_squared = radius * radius;
    // Clamp before squaring: a sphere smaller than the layer has no rock core.
    let core_radius = (radius - 1.0).max(0.0);
    let core_radius_squared = core_radius * core_radius;

    for z in 0..size[2] {
        for y in 0..size[1] {
            for x in 0..size[0] {
                let dx = x as f32 - center[0];
                let dy = y as f32 - center[1];
                let dz = z as f32 - center[2];
                let distance_squared = dx * dx + dy * dy + dz * dz;
                if distance_squared <= radius_squared {
                    // The core boundary belongs to soil; the outer boundary is solid.
                    let material = if distance_squared < core_radius_squared {
                        ROCK
                    } else {
                        SOIL
                    };
                    assert!(grid.set([x, y, z], material));
                }
            }
        }
    }
    grid
}

/// Bevy runs this once after generation and its queued insertion have completed.
/// Bevy supplies shared access to the resource declared by the parameter.
fn report_planet(planet: Res<PlanetVoxels>) {
    info!(
        "Layered sphere: center={:?}, layer={:?}, surface={:?}, outside={:?} (air={AIR})",
        planet.grid.get([4, 4, 4]),
        planet.grid.get([6, 4, 4]),
        planet.grid.get([7, 4, 4]),
        planet.grid.get([8, 4, 4]),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cargo checks a hand-countable sphere: center plus six axial neighbors.
    #[test]
    fn radius_one_has_seven_cells() {
        let grid = solid_sphere([5, 5, 5], 1.0);
        let mut occupied = 0;
        for z in 0..5 {
            for y in 0..5 {
                for x in 0..5 {
                    if grid.get([x, y, z]) != Some(AIR) {
                        occupied += 1;
                    }
                }
            }
        }
        assert_eq!(occupied, 7);
        assert_eq!(grid.get([2, 2, 2]), Some(SOIL));
        assert_eq!(grid.get([3, 2, 2]), Some(SOIL));
        assert_eq!(grid.get([3, 3, 2]), Some(AIR));
    }

    /// Cargo checks half-cell centering on all axes of a non-cubic even grid.
    #[test]
    fn even_grid_centers_between_cells() {
        let grid = solid_sphere([4, 6, 8], 0.9);
        for z in 0..8 {
            for y in 0..6 {
                for x in 0..4 {
                    let inside = (1..=2).contains(&x)
                        && (2..=3).contains(&y)
                        && (3..=4).contains(&z);
                    assert_eq!(grid.get([x, y, z]), Some(if inside { SOIL } else { AIR }));
                }
            }
        }
    }

    /// Cargo checks the core, both layer boundaries, and air along one radius.
    #[test]
    fn materials_follow_radial_boundaries() {
        let grid = solid_sphere([9, 9, 9], 3.0);
        for (x, expected) in [(4, ROCK), (5, ROCK), (6, SOIL), (7, SOIL), (8, AIR)] {
            assert_eq!(grid.get([x, 4, 4]), Some(expected));
        }
        assert_eq!(grid.get([2, 4, 4]), Some(SOIL));
        assert_eq!(grid.get([4, 6, 4]), Some(SOIL));
        assert_eq!(grid.get([4, 4, 6]), Some(SOIL));
    }

    /// Cargo checks that clamping prevents an accidental rock core in tiny spheres.
    #[test]
    fn tiny_spheres_have_only_soil() {
        for radius in [0.0, 0.5] {
            let grid = solid_sphere([3, 3, 3], radius);
            assert_eq!(grid.get([1, 1, 1]), Some(SOIL));
            assert_eq!(grid.get([2, 1, 1]), Some(AIR));
        }
    }

    /// Cargo drives a windowless app to check the producer/consumer startup path.
    #[test]
    fn startup_inserts_sphere_before_reader() {
        let mut app = App::new();
        app.add_plugins(PlanetPlugin);
        app.update();
        let planet = app.world().resource::<PlanetVoxels>();
        assert_eq!(planet.grid.get([4, 4, 4]), Some(ROCK));
        assert_eq!(planet.grid.get([7, 4, 4]), Some(SOIL));
        assert_eq!(planet.grid.get([8, 4, 4]), Some(AIR));
    }
}
```

## Verify

From the repository root:

```sh
cargo test --offline --locked --example learning
cargo run --offline --locked --example learning
```

Expect eight passing tests: three storage tests, the three adapted sphere/startup
tests, and two new material tests. The geometry tests still check occupied cells
and even-grid centering; the new tests check material boundaries and tiny radii.
The windowless startup test checks that the actual plugin still delivers data
to its reader.

Expect this message once at startup:

```text
Layered sphere: center=Some(1), layer=Some(2), surface=Some(2), outside=Some(0) (air=0)
```

The square should still rotate. Close the window to exit. There is no rendered
sphere yet. If the radius-one test expects rock, its old material assertion was
not updated: a radius-one sphere is entirely soil under this rule. An unexpected
rock center at radius zero points to the core comparison or clamp.

Tutor verification (2026-09-20): extracted this exact Rust block into a temporary
project with copied Cargo.toml/Cargo.lock and unchanged learner main.rs, scene.rs
and voxel.rs. Offline locked Cargo test for example learning compiled and passed
all eight tests using the shared target cache. Installed Bevy 0.19.0 scheduling
and Commands sources were consulted. At preparation time, no learner source edits,
actual lesson 04 implementation review, startup-log observation or GUI validation
had been performed. Later review and close evidence appear below.

## Design and concepts

> ℹ️ **Data — Shape and material are different questions**

The outer distance check answers whether a cell is occupied. The inner check
answers what occupies it. A solid sphere can contain several materials without
changing its shape or storage layout. The existing function name still describes
its geometry; it no longer implies a uniform material.

For radius 3, the radial rule is:

| Distance from center | Stored material |
| --- | --- |
| Less than 2 | ROCK (1) |
| 2 through 3, inclusive | SOIL (2) |
| Greater than 3 | AIR (0) |

The layer has radial thickness 1, sampled at cell centers. Along an axis, both
integer distances 2 and 3 qualify; this does **not** promise exactly one voxel
along every grid line. It is also not a neighbor-based exposed-surface test.
For radius at most 1, there is no rock core. Clipping an oversized sphere at the
grid boundary can expose rock because the layer follows the sphere's radius,
not the box's edges.

> ℹ️ **Bevy — Ownership and execution stay explicit**

`PlanetPlugin::build` is called during `add_plugins` and registers systems.
At Startup, Bevy calls `generate_planet`, supplies `Commands`, then applies its
queued resource insertion before calling the chained `report_planet` with
`Res<PlanetVoxels>`. This uses the default schedule's deferred-application behavior.
Generation calls `solid_sphere` directly as ordinary Rust. It runs once, while
square rotation remains an Update system that runs each frame.

```text
local VoxelGrid → queued PlanetVoxels → world-owned resource → shared reader
```

Materials are cell values inside one resource, not components or entities per
voxel. Bevy sees access to the resource as a whole; it does not schedule work
separately for rock and soil. No new ECS system is needed just to choose a byte
while constructing the grid.

> ℹ️ **Rust — Constants, expressions, and mutation**

`const ROCK: u8 = 1` names a byte value at compile time. It is not a Bevy resource
and requires no runtime insertion. These constants define the generator's
material vocabulary; `VoxelGrid` remains generic byte storage and accepts other
IDs too. Air remains the storage contract's zero value.

`let material = if ... { ROCK } else { SOIL };` uses `if` as an expression.
Both branches yield `u8`; `material` needs no `mut` because it is assigned once
per loop iteration. `let mut grid` permits calls to `set(&mut self, ...)`.
The mutable binding lets the method borrow the grid mutably; it does not make
all other references mutable. Returning `grid` moves ownership to the caller.

`mut commands: Commands` similarly allows mutation of the command wrapper.
`app: &mut App` is already a mutable reference: it permits editing the App
without a mutable binding to the reference itself. The reader's `Res` provides
shared access and cannot edit the voxel grid.

> ℹ️ **Design — Keep the next boundary small**

The chosen rule is a temporary uniform soil layer, not geology or geography.
Keep material IDs separate from display colors so a later palette can change
appearance without regenerating terrain. An enum could constrain valid material
values but would require changing the grid API or explicit conversion; byte IDs
keep the current storage lesson intact. Revisit validation when loading external
material data, and introduce a shared material module when another consumer
needs these definitions.

Choosing materials inside generation avoids a second full-grid pass. A separate
painting pass would be useful for interchangeable biome rules, at the cost of
another traversal. Neither choice has been benchmarked here. The current small,
synchronous generator still visits every cell and retains lesson 03's size and
floating-point limitations. Rendering and the geographic generator remain later
work.

## Experiment and discuss

Change only the soil thickness in `let core_radius = (radius - 1.0).max(0.0);`
from `1.0` to `2.0`. Predict the four startup samples, and also the unlogged cell
`[5, 4, 4]`. Run the app and compare the log with your prediction. Why might the
log alone fail to show that the material distribution changed?

Restore `1.0`, rerun the tests, then bring your observations or request
**Review my work**. The tests describe the original thickness and can fail during
the experiment.

## Checkpoint

- [x] Lesson prepared and isolated validation recorded.
- [x] User applied the code; reviewed with no correctness findings.
- [x] Actual exercise tests passed; user confirmed restored log and rotation.
- [x] Thickness experiment and distance/loop questions discussed.

Decision: retain plain byte storage and one owning resource; select materials
inside generation, with a fixed radial layer until geographic rules are needed.
Review (2026-09-20): actual exercise executable lines match this lesson; all eight
actual-exercise tests passed. Radius 3.0 and thickness 1.0 restored. User reported
the unchanged log during the thickness experiment; discussed its sampling limits,
squared distances, loops, radius and clipping. No assistant GUI validation.
User confirmed restored app output and continued rotation at close. Lesson complete
for this increment; no next lesson prepared.
