macro_rules! cas {
    ($a:ident, $b:ident) => {
        if $b.const_lt($a) {
            ::core::mem::swap(&mut $a, &mut $b);
        }
    };
}

macro_rules! sort3 {
    ($T:ty, $a:expr, $b:expr, $c:expr) => {{
        let (mut a, mut b, mut c): ($T, $T, $T) = ($a, $b, $c);
        $crate::card::util::cas!(a, b);
        $crate::card::util::cas!(b, c);
        $crate::card::util::cas!(a, b);
        [a, b, c]
    }};
}

macro_rules! sort4 {
    ($T:ty, $a:expr, $b:expr, $c:expr, $d:expr) => {{
        let (mut a, mut b, mut c, mut d): ($T, $T, $T, $T) = ($a, $b, $c, $d);
        $crate::card::util::cas!(a, b);
        $crate::card::util::cas!(c, d);
        $crate::card::util::cas!(a, c);
        $crate::card::util::cas!(b, d);
        $crate::card::util::cas!(b, c);
        [a, b, c, d]
    }};
}

macro_rules! sort5 {
    ($T:ty, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {{
        let (mut a, mut b, mut c, mut d, mut e): ($T, $T, $T, $T, $T) =
            ($a, $b, $c, $d, $e);
        $crate::card::util::cas!(a, b);
        $crate::card::util::cas!(d, e);
        $crate::card::util::cas!(c, e);
        $crate::card::util::cas!(c, d);
        $crate::card::util::cas!(a, d);
        $crate::card::util::cas!(a, c);
        $crate::card::util::cas!(b, e);
        $crate::card::util::cas!(b, d);
        $crate::card::util::cas!(b, c);
        [a, b, c, d, e]
    }};
}

macro_rules! const_cmp_opt {
    ($lhs:expr, $rhs:expr) => {{
        match ($lhs, $rhs) {
            (None, Some(_)) => return true,
            (Some(_), None) => return false,
            (Some(a), Some(b)) => {
                if a.const_lt(b) {
                    return true;
                }
                if b.const_lt(a) {
                    return false;
                }
            }
            (None, None) => {}
        }
    }};
}

pub(crate) use cas;
pub(crate) use const_cmp_opt;
pub(crate) use sort3;
pub(crate) use sort4;
pub(crate) use sort5;

#[cfg(test)]
mod tests {
    use crate::*;

    #[quickcheck]
    fn test_sort3(cs: CardN<3>) {
        let lhs = sort3!(Card, cs[0], cs[1], cs[2]);
        let mut rhs = [cs[0], cs[1], cs[2]];
        rhs.sort_unstable();

        assert_eq!(lhs, rhs);
    }

    #[quickcheck]
    fn test_sort4(cs: CardN<4>) {
        let lhs = sort4!(Card, cs[0], cs[1], cs[2], cs[3]);
        let mut rhs = [cs[0], cs[1], cs[2], cs[3]];
        rhs.sort_unstable();

        assert_eq!(lhs, rhs);
    }

    #[quickcheck]
    fn test_sort5(cs: CardN<5>) {
        let lhs = sort5!(Card, cs[0], cs[1], cs[2], cs[3], cs[4]);
        let mut rhs = [cs[0], cs[1], cs[2], cs[3], cs[4]];
        rhs.sort_unstable();

        assert_eq!(lhs, rhs);
    }
}
