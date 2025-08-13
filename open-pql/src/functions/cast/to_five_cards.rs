use super::*;

pub fn to_five_cards(s: &str) -> Result<Card64, RuntimeError> {
    #[inline]
    fn parse<S: FnMut(&char) -> bool, T: TryFrom<char>>(
        s: &str,
        cs: &mut Filter<Chars<'_>, S>,
    ) -> Result<T, RuntimeError> {
        cs.next().map_or_else(
            || Err(RuntimeError::RateHiHandExpectedFiveCards(s.into())),
            |c| {
                c.try_into().map_or_else(
                    |_| Err(RuntimeError::RateHiHandParseFailed(s.into())),
                    Ok,
                )
            },
        )
    }

    #[inline]
    fn next_card<T: FnMut(&char) -> bool>(
        s: &str,
        cs: &mut Filter<Chars<'_>, T>,
    ) -> Result<Card, RuntimeError> {
        Ok(Card::new(parse(s, cs)?, parse(s, cs)?))
    }

    let mut chars = s.chars().filter(|c| !c.is_whitespace());

    let mut c64 = Card64::default();

    c64.set(next_card(s, &mut chars)?);
    c64.set(next_card(s, &mut chars)?);
    c64.set(next_card(s, &mut chars)?);
    c64.set(next_card(s, &mut chars)?);
    c64.set(next_card(s, &mut chars)?);

    if chars.next().is_some() || c64.count() != 5 {
        return Err(RuntimeError::RateHiHandExpectedFiveCards(s.into()));
    }

    Ok(c64)
}
