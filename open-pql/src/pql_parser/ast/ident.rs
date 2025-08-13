use super::*;

#[derive(PartialEq, Eq, Debug, From)]
pub struct Ident<'i> {
    pub inner: &'i str,
    pub loc: (Loc, Loc),
}

#[cfg(test)]
mod tests {
    use super::{super::super::parser::*, *};

    fn i(s: &str) -> Ident<'_> {
        IdentParser::new().parse(s).unwrap()
    }

    #[test]
    fn test_ident() {
        assert_eq!(i("_"), ("_", (0, 1)).into());

        for c in "0123456789".chars() {
            assert_eq!(
                i(&format!("_{c}")),
                (format!("_{c}").as_str(), (0, 2)).into()
            );
        }

        let mut c = 'a';

        while c <= 'z' {
            assert_eq!(
                i(&format!("{c}")),
                (format!("{c}").as_str(), (0, 1)).into()
            );

            assert_eq!(
                i(&format!("{}", c.to_uppercase())),
                (format!("{}", c.to_uppercase()).as_str(), (0, 1)).into()
            );

            c = (c as u8 + 1) as char;
        }

        assert_eq!(i("__abc123"), ("__abc123", (0, 8)).into());
    }
}
