use super::*;

pub type VmProgramInner = Vec<(VmInstruction, SourceLocation)>;

#[derive(Clone, Default)]
pub struct VmProgram(pub(crate) VmProgramInner);

impl fmt::Debug for VmProgram {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.0.iter().map(|t| t.0.clone()))
            .finish()
    }
}

impl VmProgram {
    pub fn execute(&self, ctx: &mut VmExecContext) -> PQLResult<VmStackValue> {
        for (ins, loc) in &self.0 {
            match ins.execute(ctx) {
                Ok(()) => (),
                Err(err) => return Err(PQLError::from((*loc, err))),
            }
        }

        Ok(ctx.stack.pop().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let val = sval!(@long 0);
        let program = VmProgram(vec![(VmInstruction::Push(val), (0, 1))]);
        let mut ctx = VmExecContext::default();

        assert_eq!(program.execute(&mut ctx), Ok(val));
    }

    #[test]
    fn test_err() {
        let program =
            VmProgram(vec![(VmInstruction::CastNum(PQLType::LONG), (0, 1))]);
        let mut ctx = VmExecContext::default();

        assert_eq!(
            program.execute(&mut ctx),
            Err(((0, 1), InternalError::StackUnderflow).into())
        );
    }
}
