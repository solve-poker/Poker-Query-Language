// TODO: remove!; tmp implementation
#![cfg_attr(coverage_nightly, coverage(off))]

use super::*;

pub struct PQLRunner {}

/// Runs `n_trails` successful trials on its own clone of the [`Vm`]
/// (sharing `cache` with the other clones).
fn run_trials(
    mut vm: Vm,
    n_trails: usize,
    where_program: Option<&VmProgram>,
    programs: &[VmProgram],
    selectors: &[ast::Selector],
) -> PQLResult<RunnerOutput> {
    let mut rng = rand::rng();
    let mut output = RunnerOutput::new(vm.static_data.game, selectors);

    while output.n_succ < n_trails {
        if output.n_fail == n_trails {
            // TODO: fix this
            return Err(((0, 1), VmError::SamplingFailed).into());
        }

        match vm.sample(&mut rng) {
            Some(()) => {
                if let Some(wp) = where_program {
                    let keep =
                        matches!(wp.execute(&mut vm.as_context())?, VmStackValue::Bool(true));

                    if !keep {
                        //TODO: refine this
                        output.n_fail += 1;
                        continue;
                    }
                }

                for (idx, program) in programs.iter().enumerate() {
                    output.push_value(idx, program.execute(&mut vm.as_context())?);
                }
                output.n_succ += 1;
            }
            None => output.n_fail += 1,
        }
    }

    Ok(output)
}

impl PQLRunner {
    // TODO: check max selectors
    // TODO: refactor run logic
    // TODO: remove
    #[allow(clippy::missing_panics_doc)]
    pub fn try_run_stmt(
        stmt: &ast::Stmt<'_>,
        max_trials: Option<usize>,
        n_threads: Option<usize>,
    ) -> PQLResult<RunnerOutput> {
        let mut vm = Vm::from_stmt(stmt)?;

        if let Some(n) = max_trials {
            vm.static_data.n_trails = n;
        }

        let n_trails = vm.static_data.n_trails;

        let where_program = match &stmt.where_clause {
            Some(expr) => Some(vm::compile_where(&mut vm, expr)?),
            None => None,
        };

        let programs = stmt
            .selectors
            .iter()
            .map(|s| vm::compile_selector(&mut vm, s))
            .collect::<PQLResult<Vec<_>>>()?;

        let n_threads = n_threads
            .or_else(|| thread::available_parallelism().map(usize::from).ok())
            .unwrap_or(1)
            .clamp(1, n_trails.max(1));

        let outputs = thread::scope(|scope| {
            let vm = &vm;
            let programs = &programs;
            let where_program = where_program.as_ref();
            let selectors = &stmt.selectors;

            (0..n_threads)
                .map(|i| {
                    // distribute the remainder over the first few threads
                    let quota = n_trails / n_threads + usize::from(i < n_trails % n_threads);

                    scope.spawn(move || {
                        run_trials(vm.clone(), quota, where_program, programs, selectors)
                    })
                })
                .collect::<Vec<_>>()
                .into_iter()
                .map(|handle| handle.join().unwrap())
                .collect::<PQLResult<Vec<_>>>()
        })?;

        Ok(outputs
            .into_iter()
            .reduce(|mut acc, output| {
                acc.merge(output);
                acc
            })
            .unwrap())
    }

    // tmp function
    pub fn run<S: io::Write, T: io::Write>(
        src: &str,
        max_trials: Option<usize>,
        n_threads: Option<usize>,
        stream_out: &mut S,
        stream_err: &mut T,
    ) -> io::Result<()> {
        match parse_pql(src) {
            Ok(stmts) => {
                for (i, stmt) in stmts.iter().enumerate() {
                    if i > 0 {
                        writeln!(stream_out, "{:-<80}", "")?;
                    }

                    match Self::try_run_stmt(stmt, max_trials, n_threads) {
                        Ok(output) => {
                            output.report_to_stream(stmt, stream_out)?;
                            writeln!(stream_out, "{} trials", output.n_succ)?;
                        }
                        Err(err) => {
                            writeln!(stream_err, "{err:?} {}", &src[err.loc.0..err.loc.1])?;
                        }
                    }
                }
            }
            Err(err) => writeln!(stream_err, "{err:?}")?,
        }
        Ok(())
    }
}
