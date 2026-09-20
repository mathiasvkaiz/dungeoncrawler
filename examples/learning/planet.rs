use bevy::prelude::*;

use crate::voxel::VoxelGrid;

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
/// Radius and the fixed layer thicknexx are measured in cell spacings.
/// Walkthrough: start with air, find the center, then visit each cell.
/// Leave cells outside the sphere as air. Inside, choose rock for the core
/// and soil for the outer layer. Return the finished grid to the caller.
/// For radius 3, distances below 2 are rock: distances 2 through 3 are soil.
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
                    // The core boundary belongs to soil: the outer boundary is solid.
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

    // Cargo checks half-cell centering on all axes of a non-cubic even grid.
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
