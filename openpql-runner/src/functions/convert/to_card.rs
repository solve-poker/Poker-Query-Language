use super::*;

#[pqlfn]
pub fn to_card(text: &PQLString) -> Result<PQLCard, ParseError> {
    text.parse::<PQLCard>()
}
