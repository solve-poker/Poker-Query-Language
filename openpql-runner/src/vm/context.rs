use super::*;

pub struct VmExecContext<'vm> {
    pub stack: &'vm mut VmStack,
    pub heap: &'vm mut VmHeap,
    pub fn_ctx: PQLFnContext<'vm>,
}

#[cfg(test)]
impl<'c> From<&'c CompilerData<'static>> for VmExecContext<'c> {
    // TODO: replace Box::leak
    fn from(_data: &CompilerData) -> Self {
        Self {
            stack: Box::leak(Box::new(VmStack::default())),
            heap: Box::leak(Box::new(VmHeap::default())),
            fn_ctx: PQLFnContext::default(),
        }
    }
}

#[cfg(test)]
impl Default for VmExecContext<'_> {
    // TODO: replace Box::leak
    fn default() -> Self {
        Self {
            stack: Box::leak(Box::new(VmStack::default())),
            heap: Box::leak(Box::new(VmHeap::default())),
            fn_ctx: PQLFnContext::default(),
        }
    }
}
