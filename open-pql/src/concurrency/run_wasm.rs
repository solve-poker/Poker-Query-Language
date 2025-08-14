use crate::error::PQLError;

pub fn parallel_exec<T: Send + 'static>(
    procs: Vec<Box<dyn FnOnce() -> T + Send>>,
) -> Result<Vec<T>, PQLError> {
    let mut results = Vec::with_capacity(procs.len());

    for proc in procs {
        // Execute each closure sequentially
        let res = proc();
        results.push(res);
    }

    Ok(results)
}
