use super::{Loc, str};

#[derive(Clone, PartialEq, Eq, derive_more::From, derive_more::Debug)]
#[debug("{}", self.inner)]
pub struct Ident<'i> {
    pub inner: &'i str,
    pub loc: (Loc, Loc),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn assert_ident(src: &str, expected: &str) {
        let loc_start = 0;
        let loc_end = src.len();

        assert_eq!(
            parse_ident(src),
            Ok((expected, (loc_start, loc_end)).into())
        );
    }

    #[test]
    fn test_ident() {
        assert_ident("_", "_");

        for c in "0123456789".chars() {
            let input = format!("_{c}");
            assert_ident(&input, &input);
        }

        let mut c = 'a';

        while c <= 'z' {
            let lowercase = format!("{c}");
            assert_ident(&lowercase, &lowercase);

            let uppercase = format!("{}", c.to_uppercase());
            assert_ident(&uppercase, &uppercase);

            c = (c as u8 + 1) as char;
        }

        assert_ident("__abc123", "__abc123");
    }

    #[test]
    fn test_dbg() {
        assert_eq!(
            format!("{:?}", Ident::from(("content", (0, 1)))),
            "content"
        );
    }
}
