# 02 — A voxel grid owned by the world

Status: completed and reviewed for this increment (2026-09-20).
Prerequisite: lesson 01 complete; `main.rs` and `scene.rs` exist in
`examples/learning/`, with the rotating square restored.
Target: `learn/bevy-ecs`. Bevy 0.19.0; Rust edition 2024.

## Outcome

Store a small 3 × 2 × 2 voxel grid in one Bevy resource and read it from a startup
system. The terminal reports one occupied cell; the square keeps rotating.
This is the first data step toward the planet. No terrain generation yet.

## Apply it yourself

### Create `examples/learning/voxel.rs`

```rust
/// Dense, x-fastest storage. 0 means air; other bytes are material IDs.
pub struct VoxelGrid {
    size: [usize; 3],
    cells: Vec<u8>,
}

impl VoxelGrid {
    /// Called by the owner when creating a grid; initialize every cell as air.
    /// Panics for zero dimensions or a cell count that overflows usize.
    pub fn new(size: [usize; 3]) -> Self {
        assert!(size.iter().all(|&axis| axis > 0), "dimensions must be nonzero");
        let count = size[0]
            .checked_mul(size[1])
            .and_then(|area| area.checked_mul(size[2]))
            .expect("voxel count overflow");
        Self {
            size,
            cells: vec![0; count],
        }
    }

    /// Called by get/set for each access; reject coordinates outside any axis.
    fn index(&self, [x, y, z]: [usize; 3]) -> Option<usize> {
        let [width, height, depth] = self.size;
        if x >= width || y >= height || z >= depth {
            return None;
        }
        // Advance one cell along x, one row along y, or one layer along z.
        Some(x + width * (y + height * z))
    }

    /// Called by readers as needed; return a copied material ID or no valid cell.
    pub fn get(&self, position: [usize; 3]) -> Option<u8> {
        self.index(position).map(|index| self.cells[index])
    }

    /// Called by editors as needed; return false without writing if out of bounds.
    pub fn set(&mut self, position: [usize; 3], material: u8) -> bool {
        let Some(index) = self.index(position) else {
            return false;
        };
        self.cells[index] = material;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::VoxelGrid;

    /// Cargo's test runner checks the storage contract on a non-cubic grid.
    #[test]
    fn x_then_y_then_z_layout() {
        let grid = VoxelGrid::new([3, 2, 2]);
        assert_eq!(grid.index([1, 0, 0]), Some(1));
        assert_eq!(grid.index([0, 1, 0]), Some(3));
        assert_eq!(grid.index([0, 0, 1]), Some(6));
        assert_eq!(grid.index([2, 1, 1]), Some(11));
    }

    /// Cargo's test runner checks that a write changes exactly its intended cell.
    #[test]
    fn write_preserves_other_cells() {
        let mut grid = VoxelGrid::new([3, 2, 2]);
        assert!(grid.set([2, 1, 1], 7));
        for z in 0..2 {
            for y in 0..2 {
                for x in 0..3 {
                    let expected = if [x, y, z] == [2, 1, 1] { 7 } else { 0 };
                    assert_eq!(grid.get([x, y, z]), Some(expected));
                }
            }
        }
    }

    /// Cargo's test runner checks each boundary, including an overflowing input.
    #[test]
    fn invalid_coordinates_do_not_alias_valid_cells() {
        let mut grid = VoxelGrid::new([3, 2, 2]);
        for position in [[3, 0, 0], [0, 2, 0], [0, 0, 2], [usize::MAX, 0, 0]] {
            assert_eq!(grid.get(position), None);
            assert!(!grid.set(position, 9));
        }
        assert!(grid.cells.iter().all(|&cell| cell == 0));
    }
}
```

### Create `examples/learning/planet.rs`

```rust
use bevy::prelude::*;

use crate::voxel::VoxelGrid;

pub struct PlanetPlugin;

/// The ECS world owns this resource; it owns the plain Rust voxel grid.
#[derive(Resource)]
struct PlanetVoxels {
    grid: VoxelGrid,
}

impl Plugin for PlanetPlugin {
    /// Bevy calls this when main adds PlanetPlugin, before run().
    /// Insert our tiny fixture immediately and register its startup reader.
    fn build(&self, app: &mut App) {
        let mut grid = VoxelGrid::new([3, 2, 2]);
        assert!(grid.set([2, 1, 1], 1));
        app.insert_resource(PlanetVoxels { grid })
            .add_systems(Startup, report_voxel);
    }
}

/// Bevy runs this once in Startup, supplying read access to PlanetVoxels.
/// Log one cell to demonstrate that the world retained the grid after build.
fn report_voxel(planet: Res<PlanetVoxels>) {
    info!("Voxel [2, 1, 1]: {:?}", planet.grid.get([2, 1, 1]));
}
```

### Replace `examples/learning/main.rs`

Keep `scene.rs` as it is. No dependency changes are needed.

```rust
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

## Verify

From the repository root:

```sh
cargo test --offline --locked --example learning
cargo run --offline --locked --example learning
```

Expect three passing tests. The app should print `Voxel [2, 1, 1]: Some(1)` once,
alongside Bevy's usual logging, and show the rotating square. The grid is not
connected to rendering yet. Close the window to exit.

If the tests report zero tests, check `mod voxel;` and the file paths. A missing
resource error suggests that the reader was registered without the insertion.
`Some(0)` means a valid air cell; `None` means the coordinate is outside the grid.
Offline dependency errors mean the needed crates are not cached.

Tutor verification (2026-09-20): extracted these exact three Rust blocks into a
temporary project with copies of Cargo.toml/Cargo.lock and the user's existing
scene.rs. `cargo check --offline --locked --example learning` and
`cargo test --offline --locked --example learning` passed there using the shared
target cache; all three tests passed. Rust/Cargo 1.97.0. Resource and insertion
APIs were checked against installed Bevy 0.19.0 sources. The assistant did not
change user source. Later user verification is recorded in the checkpoint below.

## Design and concepts

> ℹ️ **Bevy — Data ownership is a design choice**

The ownership chain is `World → PlanetVoxels → VoxelGrid → Vec<u8>`.
The vector owns one contiguous heap allocation of material bytes. `VoxelGrid`
has no Bevy imports or traits; the `PlanetVoxels` wrapper makes it available to
systems. The world keeps one resource value of this type. Twelve cells do not
require twelve entities: individual cells need indexing, not independent ECS
identity, transforms or scheduling.

One resource fits this single-planet exercise. If multiple planets need separate
lifetimes and transforms, a grid-owning component on each planet entity is a
reasonable next ownership choice. The plain grid can remain the same.

Voxel data will be authoritative; a future image or mesh will be derived from it.
The current square is only a presentation placeholder and does not represent a
cell. Material ID 1 is a label for now; a later palette can give it an appearance.

> ℹ️ **Bevy — Configuration, execution and borrowing**

```text
main → add_plugins → PlanetPlugin::build
  create local grid → move it into the world's resource storage
  register report_voxel
run → Startup → report_voxel borrows the resource
    → Update  → existing spin_squares continues
```

`App::insert_resource` acts immediately during configuration. It differs from
queued `Commands` inside a system. `add_systems` stores the function for later
execution; it does not call it. At execution Bevy obtains the resource from the
world and supplies `Res<PlanetVoxels>` from the system's declared parameter type.
The reader needs no manual call from `main` and no entity query.

The two startup systems have no ordering dependency: the resource already exists
before either runs, and the reader does not use the camera or square. Plugin
registration order does not establish startup system execution order.
`Res` is shared read access; a future editing system would declare
`ResMut<PlanetVoxels>`. Bevy uses those accesses when scheduling systems so a
resource writer cannot run concurrently with a reader of that resource.

> ℹ️ **Rust — Owned values, mutable bindings and borrows**

`let mut grid` owns the grid and allows borrowing it mutably for `set`.
`set(&mut self, ...)` borrows that grid exclusively for the call; it does not take
ownership. `get(&self, ...)` only needs a shared borrow. `app: &mut App` lets us
modify the borrowed app without making the `app` binding itself mutable.

`PlanetVoxels { grid }` is shorthand for `PlanetVoxels { grid: grid }`. It moves
the grid into the wrapper, and insertion moves the wrapper into the world.
The original local binding can no longer be used. In the reader, `planet` is a
temporary borrow wrapper, not a copied planet. Only the returned `u8` is copied.

`[usize; 3]` is a fixed-length array, and `[x, y, z]` destructures it into three
bindings. `usize` is Rust's indexing integer; these coordinates cannot be negative.
`Self` names the current implementation type. Private fields protect the invariant
that the vector length equals the product of dimensions.

`Option<T>` distinguishes `Some(value)` from `None`. `map` transforms a present
index into a byte and leaves absence unchanged. `let Some(index) = ... else`
extracts a valid index or returns early. `|area| ...` is a closure; `and_then`
connects two checked multiplications, each of which can return `None`.
`{:?}` uses debug formatting, which is why the log includes `Some(...)`.
The `#[cfg(test)]` module exists only for test builds; `#[test]` registers each
function with Rust's test runner. Child tests can inspect their parent's private
fields, while application modules must use the grid's public methods.

> ℹ️ **Data layout — Bounds before flattening**

For a width of 3 and height of 2, x advances by 1 byte, y by 3, and z by 6.
The formula `x + width * (y + height * z)` maps a valid coordinate to one index.
Checking only whether that index fits the vector is insufficient: an invalid
x of 3 could otherwise alias the next row's first cell. The boundary test checks
this failure mode; the non-cubic dimensions help expose swapped-axis mistakes.

Dense storage is simple and uses one material byte per cell, plus vector/size
metadata. Empty cells also cost space. Sparse storage could save space for large,
mostly empty volumes but adds lookup and bookkeeping costs. Revisit storage when
real dimensions and measurements justify it; this lesson makes no speed claims.

Constructor overflow checks prevent invalid length arithmetic, not excessive
allocation. Dimensions are tiny programmer-chosen constants here. Before loading
external sizes, add a memory budget and recoverable construction errors. The
fixture in `build` is temporary; replace it with generation when that lesson is
ready. Expensive generation should not be hidden inside plugin registration.

## Experiment and discuss

Before running it, predict the output if you change only the reader's coordinate
from `[2, 1, 1]` to `[3, 0, 0]`. Why might checking only a flattened vector index
produce the wrong result? Try it, compare with your prediction, then restore the
reader. The existing boundary test is a useful clue.

## Checkpoint

- [x] Lesson prepared and isolated validation recorded.
- [x] User applied the three files; reviewed with no correctness findings.
- [x] Assistant ran the actual exercise tests; user confirmed startup log and rotation.
- [x] Bounds experiment performed and explained; further questions welcome.

Review (2026-09-20): the actual exercise compiled under
`cargo test --offline --locked --example learning`; all three tests passed.
User reported `None` for `[3, 0, 0]`; discussed per-axis bounds and next-row
aliasing. Source confirms the reader is restored to `[2, 1, 1]`. User confirmed
restored `Some(1)` output and continued rotation, then requested session close.
No assistant GUI validation. Minor cleanup: remove trailing whitespace on the blank line 23
in `planet.rs`. No exercise source was edited by the assistant.

Decision: keep the dense grid independent of Bevy and give one resource ownership
of this single planet's data. Revisit the owner for multiple planets and the
storage after measuring real workloads. Next on Continue learning: a small sphere-data lesson;
no subsequent lesson is prepared.
