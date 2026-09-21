/// Dense, x-fastest storage. 0 means air, other bytes are material IDs.
pub struct VoxelGrid {
    size: [usize; 3],
    cells: Vec<u8>,
}

impl VoxelGrid {
    /// Called by the owner when creating a grid; initialize every cell as air
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

    /// Called by readers as needed; return copied dimensions without exposing cells.
    pub fn size(&self) -> [usize; 3] {
        self.size
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
