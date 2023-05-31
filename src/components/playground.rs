use leptos::{SignalUpdate, WriteSignal};
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use super::{tile::Tile, tiles_column::draw_tiles_column};
use crate::common::{
    canvas::clear_canvas,
    constants::{
        FONT, MAX_TILES, MOVE_SIZE, SELECTION_MAX, TILE_GAP, TILE_PULL_SPEED,
        TILE_SELECTION_BORDER, TILE_SIZE, TILE_SIZE_IN_MOVES, TILE_SIZE_N_GAP,
    },
};

#[derive(Clone)]
pub struct Playground {
    tiles_cols: Vec<Vec<Tile>>,
    moves_till_tiles_addition: u16,
    selected_symbols: Vec<String>,
}

impl Playground {
    pub fn new() -> Self {
        // generating init tiles
        let tiles_cols = (0..6)
            .map(|col_i| {
                let x = (col_i * TILE_SIZE_N_GAP + TILE_GAP) as f64;
                (0..4)
                    .rev()
                    .map(|tile_i| {
                        let y = (tile_i * TILE_SIZE_N_GAP) as f64;
                        Tile::new(x, y)
                    })
                    .collect()
            })
            .collect();

        Self {
            tiles_cols,
            moves_till_tiles_addition: TILE_SIZE_IN_MOVES,
            selected_symbols: Vec::with_capacity(SELECTION_MAX),
        }
    }

    pub fn draw_playground(&self, ctx: &CanvasRenderingContext2d) {
        clear_canvas();

        // setting styles
        ctx.set_fill_style(&JsValue::from_str("aqua"));
        ctx.set_stroke_style(&JsValue::from_str("red"));
        ctx.set_line_width(TILE_SELECTION_BORDER);
        ctx.set_font(FONT);

        for col in self.tiles_cols.iter() {
            draw_tiles_column(ctx, col)
        }
    }

    pub fn move_playground(&mut self) {
        for col in self.tiles_cols.iter_mut() {
            let mut last_y = None;

            for tile in col.iter_mut().rev() {
                tile.y += MOVE_SIZE;

                if let Some(last_y) = last_y {
                    if tile.y - (TILE_SIZE_N_GAP as f64) > last_y {
                        tile.y -= MOVE_SIZE * TILE_PULL_SPEED
                    }
                }

                last_y = Some(tile.y)
            }
        }

        // checking if empty place for the next tiles line is ready
        self.moves_till_tiles_addition -= 1;
        if self.moves_till_tiles_addition == 0 {
            // adding next line
            for (i, col) in self.tiles_cols.iter_mut().enumerate() {
                // checking if we got max tiles
                if col.len() == MAX_TILES {
                    panic!()
                }

                let x = TILE_SIZE_N_GAP * i as u16 + TILE_GAP as u16;
                let new_tile = Tile::new(x.into(), f64::default());
                col.push(new_tile)
            }

            self.moves_till_tiles_addition = TILE_SIZE_IN_MOVES
        }
    }

    pub fn on_click(&mut self, x: f64, y: f64, set_score: WriteSignal<usize>) {
        self.select_symbol(x, y);

        if self.selected_symbols.len() < SELECTION_MAX {
            return;
        }

        // checking selected symbols identity
        if let Some((first, remaining)) = self.selected_symbols.split_first() {
            if remaining.iter().all(|key| *key == *first) {
                for col in self.tiles_cols.iter_mut() {
                    col.retain(|tile| !tile.is_selected)
                }

                set_score.update(|score| *score += SELECTION_MAX)
            } else {
                // unselecting
                for col in self.tiles_cols.iter_mut() {
                    for tile in col.iter_mut() {
                        tile.is_selected = false
                    }
                }
            }
        }

        self.selected_symbols.clear()
    }

    fn select_symbol(&mut self, x: f64, y: f64) {
        for col in self.tiles_cols.iter_mut() {
            for tile in col.iter_mut() {
                if !tile.is_selected
                    && tile.x < x
                    && tile.y < y
                    && tile.x + TILE_SIZE as f64 > x
                    && tile.y + TILE_SIZE as f64 > y
                {
                    tile.is_selected = true;
                    self.selected_symbols.push(tile.symbol.clone());
                    return;
                }
            }
        }
    }
}
