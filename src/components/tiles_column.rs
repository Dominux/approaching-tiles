use web_sys::CanvasRenderingContext2d;

use super::tile::{draw_tile, Tile};

pub fn draw_tiles_column(ctx: &CanvasRenderingContext2d, tiles: &Vec<Tile>) {
    for tile in tiles {
        draw_tile(ctx, tile)
    }
}
