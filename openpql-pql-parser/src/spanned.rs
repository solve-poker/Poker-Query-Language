use crate::LocInfo;

pub trait Spanned {
    fn loc(&self) -> LocInfo;
}

impl Spanned for LocInfo {
    fn loc(&self) -> LocInfo {
        *self
    }
}
