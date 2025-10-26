use super::*;

#[derive(Clone, Copy, Debug, derive_more::From, PartialEq, Eq)]
pub enum VmBinOp {
    Arith(VmBinOpArith),
    Cmp(VmBinOpCmp),
}

impl VmBinOp {
    pub fn execute(self, ctx: &mut VmExecContext) -> Result<(), PQLErrorKind> {
        match self {
            Self::Arith(op) => op.execute(ctx),
            Self::Cmp(op) => op.execute(ctx),
        }
    }

    pub(crate) fn resolve_type(
        self,
        lhs_type: PQLType,
        rhs_type: PQLType,
    ) -> Result<PQLType, PQLErrorKind> {
        match self {
            Self::Arith(op) => op.resolve_type(lhs_type, rhs_type),
            Self::Cmp(op) => op.resolve_type(lhs_type, rhs_type),
        }
    }
}

impl From<ast::BinOp> for VmBinOp {
    fn from(op: ast::BinOp) -> Self {
        match op {
            ast::BinOp::Add => Self::Arith(VmBinOpArith::Add),
            ast::BinOp::Sub => Self::Arith(VmBinOpArith::Sub),
            ast::BinOp::Mul => Self::Arith(VmBinOpArith::Mul),
            ast::BinOp::Div => Self::Arith(VmBinOpArith::Div),
            ast::BinOp::Eq => Self::Cmp(VmBinOpCmp::Eq),
            ast::BinOp::Ge => Self::Cmp(VmBinOpCmp::Ge),
            ast::BinOp::Gt => Self::Cmp(VmBinOpCmp::Gt),
            ast::BinOp::Le => Self::Cmp(VmBinOpCmp::Le),
            ast::BinOp::Lt => Self::Cmp(VmBinOpCmp::Lt),
        }
    }
}
