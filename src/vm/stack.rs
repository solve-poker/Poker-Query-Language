use super::*;

#[derive(Debug, Clone, Default)]
pub struct VmStack {
    inner: VecDeque<VmStackValue>,
}

impl VmStack {
    pub fn push(&mut self, v: VmStackValue) {
        self.inner.push_front(v);
    }

    pub fn pop(&mut self) -> Result<VmStackValue, PQLError> {
        self.inner
            .pop_front()
            .map_or_else(|| Err(InternalError::BrokenStack.into()), Ok)
    }

    pub fn downcast_pop<T>(&mut self) -> Result<T, PQLError>
    where
        T: TryFrom<VmStackValue>,
    {
        T::try_from(self.pop()?)
            .map_or_else(|_| Err(InternalError::BrokenStack.into()), Ok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_push() {
        let mut stack = VmStack::default();
        let v = VmStackValue::default_of(PQLType::Long).unwrap();

        stack.push(v);

        assert_eq!(1, stack.inner.len());
        assert_eq!(v, stack.inner[0]);
    }

    #[test]
    fn test_pop() {
        let mut stack = VmStack::default();
        let v = VmStackValue::default_of(PQLType::Long).unwrap();

        stack.push(v);

        assert_eq!(Ok(v), stack.pop());
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_downcast_pop() {
        fn downcase<T>(v: T) -> Result<T, PQLError>
        where
            T: Copy + PartialEq + fmt::Debug + TryFrom<VmStackValue>,
            VmStackValue: From<T>,
        {
            let mut stack = VmStack::default();
            let v = VmStackValue::from(v);

            stack.push(v);

            stack.downcast_pop::<T>()
        }

        fn assert_ok<T>(v: T)
        where
            T: Copy + PartialEq + fmt::Debug + TryFrom<VmStackValue>,
            VmStackValue: From<T>,
        {
            let got = downcase(v);

            assert_eq!(got, Ok(v));
        }

        assert_ok(PQLLong::default());
        assert_ok(PQLDouble::default());
        assert_ok(PQLStreet::default());

        let mut stack = VmStack::default();

        assert!(stack.downcast_pop::<PQLLong>().is_err());

        stack.push(PQLCard::default().into());
        assert!(stack.downcast_pop::<PQLLong>().is_err());
    }
}
