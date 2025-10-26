use super::*;

#[pqlfn]
pub fn to_rank(text: &PQLString) -> Result<PQLRank, ParseError> {
    text.parse::<PQLRank>()
}
