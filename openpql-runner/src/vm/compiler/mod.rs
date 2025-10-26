use super::*;

mod binop;
mod expression;
mod fncall;
mod ident;
mod numeric;
mod string;

pub use binop::*;
pub use expression::*;
pub use fncall::*;
pub use ident::*;
pub use numeric::*;
pub use string::*;

fn mk_err<I, E>(expr: &I, err: E) -> PQLError
where
    for<'a> &'a I: HasSourceLocation,
    PQLErrorKind: From<E>,
{
    (expr.loc(), err).into()
}

fn check_type<I>(expr: &I, given: PQLType, expected: PQLType) -> PQLResult<()>
where
    for<'a> &'a I: HasSourceLocation,
{
    with_loc(expr, || {
        if given.intersects(expected) {
            Ok(())
        } else {
            Err(PQLErrorKind::TypeError(given, expected))
        }
    })
}

#[derive(Debug, Clone)]
pub struct CompilerData<'vm> {
    pub static_data: &'vm VmStaticData,
    pub prog: VmProgramInner,
    pub heap: Vec<VmHeapValue>,
}

impl<'vm> CompilerData<'vm> {
    pub const fn new(static_data: &'vm VmStaticData) -> Self {
        Self {
            static_data,
            prog: vec![],
            heap: vec![],
        }
    }
}

pub fn compile_selector(
    vm: &mut Vm,
    selector: &ast::Selector,
) -> PQLResult<VmProgram> {
    let mut data = CompilerData::new(&vm.static_data);

    push_expr(&mut data, &selector.expr, selector.kind.into())?;

    let (prog, heap) = (data.prog, data.heap);

    vm.heap.extend(heap);

    // TODO: ref offset

    Ok(VmProgram(prog))
}

#[cfg(test)]
impl CompilerData<'_> {
    // TODO: replace Box::leak
    pub fn default() -> Self {
        let static_data = VmStaticData::default();

        Self {
            static_data: Box::leak(Box::new(static_data)),
            prog: vec![],
            heap: vec![],
        }
    }
}

#[cfg(test)]
pub fn assert_expr_err<E>(
    expected_type: PQLType,
    src: &str,
    err: E,
    err_src: &str,
) where
    PQLErrorKind: From<E>,
{
    let expr = parse_expr(src).unwrap();

    let mut data = CompilerData::default();

    let pos_s = src.find(err_src).unwrap();
    let pos_e = pos_s + err_src.len();

    let res = push_expr(&mut data, &expr, expected_type);

    assert_eq!(res, Err(((pos_s, pos_e), err).into()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn assert_err<E>(src: &str, err: E, err_src: &str)
    where
        PQLErrorKind: From<E>,
    {
        let sel = parse_selector(src).unwrap();
        let mut vm = Vm::default();

        let res = compile_selector(&mut vm, &sel);

        let pos_s = src.find(err_src).unwrap();
        let pos_e = pos_s + err_src.len();
        let loc = (pos_s, pos_e);

        assert_eq!(res.unwrap_err(), mk_err(&loc, err));
    }

    fn assert_ok(src: &str) {
        assert!(
            compile_selector(&mut Vm::default(), &parse_selector(src).unwrap())
                .is_ok()
        );
    }

    #[test]
    fn test_selector_type() {
        assert_ok("max(5.0)");
        assert_ok("min(3.5)");
        assert_ok("avg(2.5)");
        //assert_ok("count(true)");

        assert_err(
            "count(5.0)",
            PQLErrorKind::TypeError(PQLType::DOUBLE, PQLType::BOOLEAN),
            "5.0",
        );

        assert_err(
            "avg(1 = 1)",
            PQLErrorKind::TypeError(PQLType::BOOLEAN, PQLType::NUMERIC),
            "1 = 1",
        );
    }
}
