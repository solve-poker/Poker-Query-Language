use std::thread::{spawn, JoinHandle};

use crate::{error::PQLError, InternalError};

pub fn parallel_exec<T: Send + 'static>(
    procs: Vec<Box<dyn FnOnce() -> T + Send>>,
) -> Result<Vec<T>, PQLError> {
    let mut handles = vec![];

    for proc in procs {
        handles.push(spawn(proc));
    }

    handles
        .into_iter()
        .map(JoinHandle::join)
        .collect::<Result<Vec<_>, _>>()
        .map_or_else(|_| Err(InternalError::ThreadJoinFailed.into()), Ok)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_parallel_exec() {
        let procs =
            (0..4).map(|i| Box::new(move || i) as _).collect::<Vec<_>>();

        assert_eq!(parallel_exec(procs).unwrap(), [0, 1, 2, 3]);

        let procs = vec![Box::new(|| panic!("should panic")) as _];

        assert_eq!(
            parallel_exec(procs),
            Err(InternalError::ThreadJoinFailed.into())
        );
    }
}
