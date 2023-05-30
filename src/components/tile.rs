use web_sys::CanvasRenderingContext2d;

use crate::common::constants::{TILE_GAP, TILE_HALF_SIZE, TILE_SIZE};

pub fn draw_tile(ctx: &CanvasRenderingContext2d, tile: &Tile) {
    ctx.fill_rect(tile.x, tile.y, TILE_SIZE.into(), TILE_SIZE.into());
    let _ = ctx.fill_text(
        &tile.symbol,
        tile.x + TILE_GAP as f64,
        tile.y + TILE_HALF_SIZE + (TILE_GAP * 2) as f64,
    );

    if tile.is_selected {
        ctx.stroke_rect(tile.x, tile.y, TILE_SIZE.into(), TILE_SIZE.into())
    }
}

#[derive(Clone)]
pub struct Tile {
    pub x: f64,
    pub y: f64,
    pub symbol: String,
    pub is_selected: bool,
}

impl Tile {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            symbol: "🍆".to_string(),
            is_selected: false,
        }
    }
}
