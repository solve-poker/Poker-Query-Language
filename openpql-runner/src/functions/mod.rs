use super::*;

mod board;
mod context;
mod convert;
mod equity;
mod hand_category;
mod hand_type;
mod outcome;
mod outs;
mod range;
mod rank;
mod rating;

pub use board::*;
#[cfg(test)]
pub use context::tests::TestPQLFnContext;
pub use context::*;
pub use convert::*;
pub use equity::*;
pub use hand_category::*;
pub use hand_type::*;
pub use outcome::*;
pub use outs::*;
pub use range::*;
pub use rank::*;
pub use rating::*;

pub trait PQLFn: fmt::Debug {
    fn arg_types(&self) -> Vec<PQLType>;
    fn rtn_type(&self) -> PQLType;
    fn execute(
        &self,
        ctx: &mut VmExecContext,
    ) -> Result<VmStackValue, PQLErrorKind>;
}

impl FromStr for &dyn PQLFn {
    type Err = PQLErrorKind;

    pqlfn_fromstr!(Err(PQLErrorKind::UnrecognizedFunction));
}
