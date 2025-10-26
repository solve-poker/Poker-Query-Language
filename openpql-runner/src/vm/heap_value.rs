use super::*;

#[derive(Clone, Debug, derive_more::From)]
pub enum VmHeapValue {
    Str(String),
    Range(PQLRange),
    BoardRange(PQLBoardRange),
}

impl VmHeapValue {
    pub fn as_any(&self) -> &dyn Any {
        match &self {
            Self::Str(inner) => inner as &dyn Any,
            Self::Range(inner) => inner as &dyn Any,
            Self::BoardRange(inner) => inner as &dyn Any,
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
impl VmHeapValue {
    pub fn is_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Str(l), Self::Str(r)) => l == r,
            (Self::Range(l), Self::Range(r)) => l.src_eq(r),
            (Self::BoardRange(l), Self::BoardRange(r)) => l.src_eq(r),
            _ => false,
        }
    }
}
