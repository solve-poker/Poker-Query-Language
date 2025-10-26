// TODO: remove!; tmp implementation
#![cfg_attr(coverage_nightly, coverage(off))]

use super::*;

pub struct PQLRunner {}

impl PQLRunner {
    // TODO: check max selectors
    // TODO: refactor run logic
    // TODO: remove
    #[allow(clippy::missing_panics_doc)]
    pub fn try_run_stmt(stmt: &ast::Stmt<'_>) -> PQLResult<RunnerOutput> {
        let mut rng = rand::rng();
        let mut vm = Vm::from_stmt(stmt)?;
        let n_trails = vm.static_data.n_trails;
        let game = vm.static_data.game;

        let mut output = RunnerOutput::new(game, &stmt.selectors);

        for (idx, selector) in stmt.selectors.iter().enumerate() {
            let program = vm::compile_selector(&mut vm, selector)?;
            while output.n_succ < n_trails {
                if output.n_fail == n_trails {
                    // TODO: fix this
                    return Err(((0, 1), VmError::SamplingFailed).into());
                }

                match vm.sample(&mut rng) {
                    Some(()) => {
                        output.push_value(
                            idx,
                            program.execute(&mut vm.as_context())?,
                        );
                        output.n_succ += 1;
                    }
                    None => output.n_fail += 1,
                }
            }
        }

        Ok(output)
    }

    // tmp function
    pub fn run<S: io::Write, T: io::Write>(
        src: &str,
        stream_out: &mut S,
        stream_err: &mut T,
    ) -> io::Result<()> {
        match parse_pql(src) {
            Ok(stmts) => {
                for (i, stmt) in stmts.iter().enumerate() {
                    if i > 0 {
                        writeln!(stream_out, "{:-<80}", "")?;
                    }

                    match Self::try_run_stmt(stmt) {
                        Ok(output) => {
                            output.report_to_stream(stmt, stream_out)?;
                            writeln!(stream_out, "{} trials", output.n_succ)?;
                        }
                        Err(err) => {
                            writeln!(
                                stream_err,
                                "{err:?} {}",
                                &src[err.loc.0..err.loc.1]
                            )?;
                        }
                    }
                }
            }
            Err(err) => writeln!(stream_err, "{err:?}")?,
        }
        Ok(())
    }
}
