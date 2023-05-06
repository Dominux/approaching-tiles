use std::marker::PhantomData;

use crate::{
    models::{Tile, TilesColumn, TilesPlayground},
    types::{ColumnLength, KeyGen, TileID},
};

// Internal states
pub(crate) struct Started;
pub(crate) struct Ended;

/// Internal domain structure
///
/// It is a game state storage, not a whole game controller,
/// since it does not have any internal timer that is needed to control the game process
pub(crate) struct Game<State = Started> {
    playground: TilesPlayground,
    state: PhantomData<State>,
    keygen: KeyGen,
    score: TileID,
}

impl Game {
    pub(crate) fn new(init_len: ColumnLength, cols_count: ColumnLength, keygen: KeyGen) -> Self {
        // building playground
        let playground = {
            let mut next_id = 0;
            let cols = (0..cols_count)
                .map(|_| {
                    TilesColumn::new(
                        (0..init_len)
                            .map(|_| {
                                let tile = Tile::new(next_id, keygen());
                                next_id += 1;
                                tile
                            })
                            .collect(),
                    )
                })
                .collect();
            TilesPlayground::new(cols)
        };

        Self {
            playground,
            state: PhantomData,
            keygen,
            score: TileID::default(),
        }
    }
}

impl Game<Started> {
    /// Get current game score
    pub fn get_score(&self) -> TileID {
        self.score
    }
}
