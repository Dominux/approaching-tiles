use std::{cell::RefCell, rc::Rc};

use web_sys::CanvasRenderingContext2d;

use super::{tile::Tile, tiles_column::draw_tiles_column};
use crate::common::constants::{MOVE_SIZE, TILE_GAP, TILE_SIZE_IN_MOVES, TILE_SIZE_N_GAP};

pub struct Playground {
    tiles_cols: Rc<RefCell<Vec<Vec<Tile>>>>,
    moves_till_tiles_addition: u16,
}

impl Playground {
    pub fn new() -> Self {
        // generating init tiles
        let tiles_cols = (0..6)
            .map(|col_i| {
                let y = (col_i * TILE_SIZE_N_GAP) as f64;
                (0..6)
                    .map(|tile_i| {
                        let x = (tile_i * TILE_SIZE_N_GAP + TILE_GAP) as f64;
                        Tile::new(x, y)
                    })
                    .collect()
            })
            .collect();
        let tiles_cols = Rc::new(RefCell::new(tiles_cols));

        Self {
            tiles_cols,
            moves_till_tiles_addition: TILE_SIZE_IN_MOVES,
        }
    }

    pub fn draw_playground(&self, ctx: &CanvasRenderingContext2d) {
        Self::clear_canvas(ctx);

        for col in self.tiles_cols.borrow().iter() {
            draw_tiles_column(ctx, col)
        }
    }

    pub fn move_playground(&mut self) {
        for col in self.tiles_cols.borrow_mut().iter_mut() {
            for tile in col.iter_mut() {
                tile.y += MOVE_SIZE
            }
        }

        // checking if empty place for the next tiles line is ready
        self.moves_till_tiles_addition -= 1;
        if self.moves_till_tiles_addition == 0 {
            // adding next line
            for (i, col) in self.tiles_cols.borrow_mut().iter_mut().enumerate() {
                let x = TILE_SIZE_N_GAP * i as u16 + TILE_GAP as u16;
                let new_tile = Tile::new(x.into(), 0.0);
                col.push(new_tile)
            }

            self.moves_till_tiles_addition = TILE_SIZE_IN_MOVES
        }
    }

    fn clear_canvas(ctx: &CanvasRenderingContext2d) {
        let canvas = ctx.canvas().unwrap();
        canvas.set_width(canvas.width())
    }
}
