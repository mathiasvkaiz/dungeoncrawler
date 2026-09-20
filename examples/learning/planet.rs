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
