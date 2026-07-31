/// Pixel size of one tile in the spritesheet.
pub const TILE_PIXEL_SIZE: u32 = 32;

/// How many tiles wide/tall the world should be.
pub const MAP_TILES_X: u32 = 30;
pub const MAP_TILES_Y: u32 = 20;
pub const MAP_LAYERS: u32 = 5;

// Actual Width and Height of the window
pub const WINDOW_WIDTH: u32 = MAP_TILES_X * TILE_PIXEL_SIZE;
pub const WINDOW_HEIGHT: u32 = MAP_TILES_Y * TILE_PIXEL_SIZE;

// tilemap number of rows and cols
pub const ATLAS_COLUMNS: u32 = 8;
pub const ATLAS_ROWS: u32 = 10;