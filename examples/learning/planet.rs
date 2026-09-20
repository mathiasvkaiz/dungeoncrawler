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

/// Called by generate_planet or tests; return fresh, centered sphere data.
/// Radius is in cell spacings. Oversized spheres are clipped by the grid.
///
/// Simple walkthrough:
/// 1. Check that the radius is valid and non-negative.
/// 2. Create an empty grid: every cell starts as 0 (air).
/// 3. Find its center. For [9, 9, 9], this is [4, 4, 4].
/// 4. Visit every cell using the z, y and x loops.
/// 5. Calculate its offsets from the center: dx, dy and dz.
/// 6. Set cells within or on the radius to 1 (solid); leave the others as air.
///    Compare squared distances to avoid calculating a square root.
/// 7. Return the filled grid.
///
/// For example, [7, 4, 4] is 3 cell spacings from the center:
/// solid with radius 3, air with radius 2. Think of a ball inside a box,
/// with air filling the space around the ball.
///
/// Panics for invalid dimensions (via VoxelGrid) or a non-finite/negative radius.
fn solid_sphere(size: [usize; 3], radius: f32) -> VoxelGrid {
    assert!(radius.is_finite() && radius >= 0.0, "invalid radius");
    let mut grid = VoxelGrid::new(size);
    // Integer coordinates denote cell centers; even dimensions center between cells.
    let center = size.map(|axis| (axis as f32 - 1.0) * 0.5);
    let radius_squared = radius * radius;

    for z in 0..size[2] {
        for y in 0..size[1] {
            for x in 0..size[0] {
                // Convert before subtracting: offsets can be negative.
                let dx = x as f32 - center[0];
                let dy = y as f32 - center[1];
                let dz = z as f32 - center[2];
                if dx * dx + dy * dy + dz * dz <= radius_squared {
                    // These loop bounds guarantee that the write is valid.
                    assert!(grid.set([x, y, z], 1));
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
        "Sphere: center={:?}, surface={:?}, outside={:?}",
        planet.grid.get([4, 4, 4]),
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
                    if grid.get([x, y, z]) == Some(1) {
                        occupied += 1;
                    }
                }
            }
        }
        assert_eq!(occupied, 7);
        assert_eq!(grid.get([2, 2, 2]), Some(1));
        assert_eq!(grid.get([3, 2, 2]), Some(1));
        assert_eq!(grid.get([3, 3, 2]), Some(0));
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
                    assert_eq!(grid.get([x, y, z]), Some(if inside { 1 } else { 0 }));
                }
            }
        }
    }

    /// Cargo drives a windowless app to check the producer/consumer startup path.
    #[test]
    fn startup_inserts_sphere_before_reader() {
        let mut app = App::new();
        app.add_plugins(PlanetPlugin);
        app.update();
        let planet = app.world().resource::<PlanetVoxels>();
        assert_eq!(planet.grid.get([4, 4, 4]), Some(1));
        assert_eq!(planet.grid.get([7, 4, 4]), Some(1));
        assert_eq!(planet.grid.get([8, 4, 4]), Some(0));
    }
}
