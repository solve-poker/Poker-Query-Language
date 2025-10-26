use super::*;

#[cfg(test)]
#[macro_export]
macro_rules! stack {
    [$(@$prefix:ident $val:expr),+ $(,)?] => {
        VmStack::from(vec![$(sval!(@$prefix $val)),+])
    };
}

/// Virtual Stack
/// frame markers are not needed since there are no variables
#[derive(Clone, Debug, Default, PartialEq, derive_more::From)]
pub struct VmStack {
    inner: Vec<VmStackValue>,
}

impl VmStack {
    pub fn push(&mut self, stack_value: VmStackValue) {
        self.inner.push(stack_value);
    }

    pub fn pop(&mut self) -> Result<VmStackValue, PQLErrorKind> {
        self.inner
            .pop()
            .map_or_else(|| Err(InternalError::StackUnderflow.into()), Ok)
    }

    pub(crate) fn downcast_pop<T>(&mut self) -> T
    where
        VmStackValue: TryInto<T>,
        <VmStackValue as convert::TryInto<T>>::Error: fmt::Debug,
    {
        // value validity is guaranteed by static analysis `VmProgram::push_fncall`
        self.inner.pop().unwrap().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downcast_pop() {
        let mut stack = stack![@long 1];

        assert_eq!(stack.downcast_pop::<PQLLong>(), 1);
    }

    #[test]
    fn test_pop_err() {
        let mut stack = VmStack::default();

        assert_eq!(
            stack.pop().unwrap_err(),
            InternalError::StackUnderflow.into()
        );
    }
}
