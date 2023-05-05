use std::marker::PhantomData;

use crate::{
    models::{Tile, TilesColumn, TilesPlayground},
    types::{ColumnLength, KeyGen},
};

pub(crate) struct NotStarted;
pub(crate) struct Started;
pub(crate) struct Ended;

/// Internal domain structure
pub(crate) struct Game<State = NotStarted> {
    pub(crate) playground: TilesPlayground,
    state: PhantomData<State>,
    keygen: KeyGen,
    score: usize,
}

impl Game {
    pub(crate) fn new(init_len: ColumnLength, cols_count: ColumnLength, keygen: KeyGen) -> Self {
        // building playground
        let cols = (0..cols_count)
            .map(|_| TilesColumn::new((0..init_len).map(|_| Tile::new(keygen())).collect()))
            .collect();
        let playground = TilesPlayground::new(cols);

        Self {
            playground,
            state: PhantomData,
            keygen,
            score: usize::default(),
        }
    }
}

impl Game<NotStarted> {
    pub(crate) fn start(self) -> Game<Started> {
        // TODO: add some time logic

        Game {
            playground: self.playground,
            state: PhantomData,
            keygen: self.keygen,
            score: self.score,
        }
    }
}
