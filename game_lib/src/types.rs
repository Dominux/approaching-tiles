/// Using it since it needs the smallest amount of RAM - 1 byte,
/// easy to use, to represent and to work with
pub type TileKey = u8;

pub type ColumnLength = u8;

/// Type of functions to generate tile keys
pub(crate) type KeyGen = fn() -> TileKey;
