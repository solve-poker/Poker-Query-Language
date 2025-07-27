use super::CachedChecker;

pub type PQLBoardRange = CachedChecker<5, true>;

impl Default for PQLBoardRange {
    fn default() -> Self {
        Self::from_src("*").unwrap()
    }
}
