use crate::LocInfo;

pub trait Spanned {
    fn loc(&self) -> LocInfo;
}
