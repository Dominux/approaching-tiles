#[derive(Clone, Debug)]
pub enum CheckingResult {
    Delete,
    Leave,
    Nothing,
}

impl Default for CheckingResult {
    fn default() -> Self {
        Self::Nothing
    }
}
