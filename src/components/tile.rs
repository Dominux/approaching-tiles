use web_sys::CanvasRenderingContext2d;

use crate::common::{
    canvas::convert_to_canvas_coords,
    constants::{TILE_GAP, TILE_HALF_SIZE, TILE_SIZE},
    symbols::get_random_symbol,
};

pub fn draw_tile(ctx: &CanvasRenderingContext2d, tile: &Tile) {
    let x1 = convert_to_canvas_coords(tile.x);
    let y1 = convert_to_canvas_coords(tile.y);
    let x2 = convert_to_canvas_coords(TILE_SIZE);
    let y2 = x2;

    ctx.fill_rect(x1, y1, x2, y2);
    let _ = ctx.fill_text(
        &tile.symbol,
        convert_to_canvas_coords(tile.x + TILE_GAP),
        convert_to_canvas_coords(tile.y + TILE_HALF_SIZE + TILE_GAP * 2),
    );

    if tile.is_selected {
        ctx.stroke_rect(x1, y1, x2, y2)
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub x: u16,
    pub y: u16,
    pub symbol: String,
    pub is_selected: bool,
}

impl Tile {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x,
            y,
            symbol: get_random_symbol(),
            is_selected: false,
        }
    }
}
