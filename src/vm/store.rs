use super::*;

#[derive(Debug, Clone, Default)]
pub struct VmStore {
    pub(crate) inner: Vec<VmValue>,
}

impl VmStore {
    pub fn try_push(&mut self, v: VmValue) -> Result<VmStoreVarIdx, PQLError> {
        let idx = self.inner.len().try_into()?;

        self.inner.push(v);

        Ok(idx)
    }

    fn get(&self, i: VmStoreVarIdx) -> &VmValue {
        &self.inner[i.to_usize()]
    }

    fn get_mut(&mut self, i: VmStoreVarIdx) -> &mut VmValue {
        &mut self.inner[i.to_usize()]
    }

    pub fn downcast_get<'a, T>(
        &'a self,
        i: VmStoreVarIdx,
    ) -> Result<T, PQLError>
    where
        T: TryFrom<&'a VmValue>,
    {
        T::try_from(self.get(i))
            .map_or_else(|_| Err(InternalError::InvalidVmValue.into()), Ok)
    }

    pub fn downcast_get_mut<'a, T>(
        &'a mut self,
        i: VmStoreVarIdx,
    ) -> Result<T, PQLError>
    where
        T: TryFrom<&'a mut VmValue>,
    {
        T::try_from(self.get_mut(i))
            .map_or_else(|_| Err(InternalError::InvalidVmValue.into()), Ok)
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    impl VmStore {
        pub fn new_packed() -> Self {
            Self {
                inner: vec![
                    VmStackValue::from(PQLStreet::default()).into();
                    u8::MAX as usize + 1
                ],
            }
        }
    }

    #[test]
    fn test_try_push() {
        assert!(matches!(
            VmStore::new_packed().try_push(VmValue::Str(String::new())),
            Err(PQLError::TooManyVariables)
        ));
    }

    #[test]
    fn test_downcast_get() {
        let mut i = PQLLong::default();
        let v: VmValue = VmStackValue::from(i).into();
        let mut store = VmStore::default();
        let idx = store.try_push(v).unwrap();

        assert_eq!(&i, store.downcast_get::<&PQLLong>(idx).unwrap());
        assert_eq!(
            &mut i,
            store.downcast_get_mut::<&mut PQLLong>(idx).unwrap()
        );

        assert!(store.downcast_get::<&PQLString>(idx).is_err());
        assert!(store.downcast_get_mut::<&mut PQLString>(idx).is_err());
    }
}
