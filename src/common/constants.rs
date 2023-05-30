pub const TILE_SIZE: u16 = 70;
pub const TILE_GAP: u16 = 5;
pub const TILE_SIZE_N_GAP: u16 = TILE_SIZE + TILE_GAP;
pub const MOVE_SIZE: f64 = 0.3;
pub const TILE_SIZE_IN_MOVES: u16 = (TILE_SIZE_N_GAP as f64 / MOVE_SIZE) as u16;
