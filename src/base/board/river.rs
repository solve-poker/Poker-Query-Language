use super::{Card, From, Into};

#[derive(Clone, Copy, Debug, Into, From, Default, Eq, PartialEq)]
pub struct River(pub Card);
