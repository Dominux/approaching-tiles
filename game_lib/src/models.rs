use crate::types::{ColumnLength, TileID, TileKey};

/// Atomic game unit
#[derive(Debug, Clone)]
pub struct Tile {
    pub id: TileID,
    pub key: TileKey,
}

impl Tile {
    pub fn new(id: TileID, key: TileKey) -> Self {
        Self { id, key }
    }
}

/// Column of tiles
pub struct TilesColumn(pub Vec<Tile>);

impl TilesColumn {
    pub(crate) fn new(tiles: Vec<Tile>) -> Self {
        Self(tiles)
    }

    #[inline]
    fn len(&self) -> ColumnLength {
        self.0.len() as ColumnLength
    }

    #[inline]
    fn push(&mut self, tile: Tile) {
        self.0.push(tile)
    }

    /// Removes given indeces
    ///
    /// In case of out of range indeces, simply skips them
    fn remove_indeces(&mut self, indeces: Vec<ColumnLength>) {
        self.0 = self
            .0
            .iter()
            .rev()
            .enumerate()
            .filter(|(i, _)| !indeces.contains(&(*i as ColumnLength)))
            .map(|(_, tile)| tile.clone())
            .rev()
            .collect();
    }
}

/// Playground of tiles columns
pub struct TilesPlayground(pub Vec<TilesColumn>);

impl TilesPlayground {
    pub(crate) fn new(tiles_columns: Vec<TilesColumn>) -> Self {
        Self(tiles_columns)
    }

    /// The longest column defines the whole playground length
    ///
    /// In case of empty playground returns 0
    #[inline]
    pub(crate) fn len(&self) -> ColumnLength {
        self.0.iter().map(|col| col.len()).max().unwrap_or(0)
    }

    #[inline]
    pub(crate) fn push_line(&mut self, line: Vec<Tile>) {
        let mut iter = line.into_iter();
        for col in self.0.iter_mut() {
            col.push(iter.next().unwrap())
        }
    }
}
