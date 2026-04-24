use crate::LocInfo;

/// AST node carrying a source span.
pub trait Spanned {
    /// Returns the start and end byte offsets of the node.
    fn loc(&self) -> LocInfo;
}

impl Spanned for LocInfo {
    fn loc(&self) -> LocInfo {
        *self
    }
}
