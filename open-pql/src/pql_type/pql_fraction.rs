// TODO: remove later
#![allow(clippy::missing_panics_doc)]

use super::*;

type Inner = u16;
type Bigger = u32;

#[derive(Copy, Clone, Debug)]
pub struct PQLFraction {
    pub num: Inner,
    pub den: Inner,
}

impl PQLFraction {
    pub const fn new(num: Inner, den: Inner) -> Self {
        assert!(num < 65);
        assert!(den < 65);

        Self { num, den }
    }

    pub const fn zero() -> Self {
        Self::new(0, 1)
    }

    pub fn to_f64(self) -> f64 {
        f64::from(self.num) / f64::from(self.den)
    }
}

#[inline]
fn calc_ad_bc(f1: PQLFraction, f2: PQLFraction) -> (Bigger, Bigger) {
    let ad = Bigger::from(f1.num) * Bigger::from(f2.den);
    let bc = Bigger::from(f1.den) * Bigger::from(f2.num);

    (ad, bc)
}

#[tailcall]
fn gcd(m: Inner, n: Inner) -> Inner {
    if m == 0 { n } else { gcd(n % m, m) }
}

#[tailcall]
fn gcd_big(m: Bigger, n: Bigger) -> Bigger {
    if m == 0 { n } else { gcd_big(n % m, m) }
}

#[inline]
fn add(a: PQLFraction, b: PQLFraction) -> PQLFraction {
    let g = gcd(a.den, b.den);
    if g == 1 {
        PQLFraction {
            num: a.num * b.den + b.num * a.den,
            den: a.den * b.den,
        }
    } else {
        let g_big = Bigger::from(g);
        let den: Bigger = Bigger::from(a.den) * Bigger::from(b.den) / g_big;
        let (ad, bc) = calc_ad_bc(a, b);
        let num = (ad + bc) / g_big;

        let g1 = gcd_big(num, g_big);

        if g1 == 1 {
            PQLFraction {
                num: Inner::try_from(num).unwrap(),
                den: Inner::try_from(den).unwrap(),
            }
        } else {
            PQLFraction {
                num: Inner::try_from(num / g1).unwrap(),
                den: Inner::try_from(den / g1).unwrap(),
            }
        }
    }
}

impl PartialEq for PQLFraction {
    fn eq(&self, other: &Self) -> bool {
        let (ad, bc) = calc_ad_bc(*self, *other);
        ad == bc
    }
}

impl PartialOrd for PQLFraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (ad, bc) = calc_ad_bc(*self, *other);

        ad.partial_cmp(&bc)
    }
}

impl Add for PQLFraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        add(self, rhs)
    }
}

impl AddAssign for PQLFraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{Arbitrary, TestResult};

    use self::PQLFraction as F;
    use super::*;

    impl Arbitrary for PQLFraction {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let i = u8::arbitrary(g);
            let j = u8::arbitrary(g);

            Self {
                num: Inner::from(i >> 2),
                den: Inner::from(j >> 2),
            }
        }
    }

    fn assert_feq(f1: f64, f2: f64) {
        assert!(
            (f1 - f2).abs() <= f64::EPSILON,
            r"assertion `left == right` failed
  left: {f1:?}
 right: {f2:?}"
        );
    }

    const fn is_fraction(v: PQLFraction) -> bool {
        v.den > v.num
    }

    #[test]
    fn test_eq() {
        assert_eq!(F::new(1, 4), F::new(2, 8));
    }

    #[test]
    fn test_partial_cmp() {
        assert!(F::new(3, 8) > F::new(1, 4));
    }

    #[quickcheck]
    fn test_add(a: PQLFraction, b: PQLFraction) -> TestResult {
        if !is_fraction(a) || !is_fraction(b) {
            return TestResult::discard();
        }

        assert_feq((a + b).to_f64(), a.to_f64() + b.to_f64());

        let mut v = PQLFraction::zero();
        v += a;
        v += b;
        assert_eq!(v, a + b);

        TestResult::passed()
    }
}
