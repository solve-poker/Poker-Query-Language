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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locinfo_loc() {
        let li: LocInfo = (3, 7);
        assert_eq!(li.loc(), (3, 7));
    }
}
