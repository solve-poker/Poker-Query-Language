use crate::{
    Street,
    tree::{Action, History},
};

impl From<&History> for Street {
    fn from(h: &History) -> Self {
        match h.iter().filter(|&a| matches!(*a, Action::Chance)).count() {
            0..=1 => Self::Preflop,
            2 => Self::Flop,
            3 => Self::Turn,
            _ => Self::River,
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod tests {
    use crate::*;

    #[test]
    fn test_from_history() {
        assert_eq!(Street::from(&history!()), Street::Preflop);
        assert_eq!(Street::from(&history!(c, 2, 2)), Street::Preflop);
        assert_eq!(Street::from(&history!(c, 2, 2, c)), Street::Flop);
        assert_eq!(Street::from(&history!(c, 2, 2, c, c)), Street::Turn);
        assert_eq!(Street::from(&history!(c, 2, 2, c, c, c)), Street::River);
    }
}
