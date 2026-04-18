use super::*;

#[derive(Clone, Copy, Debug, derive_more::From, PartialEq, Eq)]
pub enum VmUnaryOp {
    Logic(VmUnaryOpLogic),
}

impl VmUnaryOp {
    pub fn execute(self, ctx: &mut VmExecContext) -> Result<(), PQLErrorKind> {
        match self {
            Self::Logic(op) => op.execute(ctx),
        }
    }

    pub(crate) fn resolve_type(
        self,
        arg_type: PQLType,
    ) -> Result<PQLType, PQLErrorKind> {
        match self {
            Self::Logic(op) => op.resolve_type(arg_type),
        }
    }
}

impl From<ast::UnaryOp> for VmUnaryOp {
    fn from(op: ast::UnaryOp) -> Self {
        match op {
            ast::UnaryOp::Not => Self::Logic(VmUnaryOpLogic::Not),
        }
    }
}
