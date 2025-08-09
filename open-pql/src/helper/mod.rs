#[cfg(feature = "x86")]
#[cfg(debug_assertions)]
mod mm256;

#[cfg(feature = "x86")]
#[cfg(debug_assertions)]
#[allow(unused)]
pub use mm256::{Bits16, View16, dbg_mm256};
