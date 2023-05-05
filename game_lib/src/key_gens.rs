use crate::types::TileKey;

/// Generate random tile key
pub(crate) fn gen_random_tilekey() -> TileKey {
    fastrand::u8(..)
}
