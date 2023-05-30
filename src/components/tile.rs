use web_sys::CanvasRenderingContext2d;

use crate::common::constants::TILE_SIZE;

pub fn draw_tile(ctx: &CanvasRenderingContext2d, tile: &Tile) {
    ctx.fill_rect(tile.x, tile.y, TILE_SIZE.into(), TILE_SIZE.into());

    if tile.is_selected {
        ctx.stroke_rect(tile.x, tile.y, TILE_SIZE.into(), TILE_SIZE.into())
    }
}

#[derive(Clone)]
pub struct Tile {
    pub x: f64,
    pub y: f64,
    pub is_selected: bool,
}

impl Tile {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            is_selected: false,
        }
    }
}
