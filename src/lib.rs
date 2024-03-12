use num_bigint::BigInt;
use num_traits::Zero;
use std::{cmp::PartialEq, ops::{Add, Mul, Neg}};

const P: &str = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
const R: &str = "3fffffffffffffffffffffffffffffffffffffffffffffffffffffff7cca23e9c44edb49aed63690216cc2728dc58f552378c292ab5844f3";
// const D: &str = "-39081";
const D: &str = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffffffffffffffffffffffffffffffffffffffffffffffff6756";

// A macro to create a BigInt from a string literal
#[macro_export]
macro_rules! bi {
    ($x: expr) => {
        $crate::bi!($x, 10)
    };
    ($x: expr, $base: literal) => {
        num_bigint::BigInt::parse_bytes($x.as_bytes(), $base).unwrap()
    }
}

pub struct Point {
    x: BigInt,
    y: BigInt,
}

impl Point {
    pub fn identity() -> Self {
        Point {
            x: BigInt::from(0),
            y: BigInt::from(1),
        }
    }

    pub fn new(x: BigInt, y: BigInt) -> Self {
        Point { x, y }
    }

    pub fn compute_y(_x: BigInt) -> BigInt {
        todo!("Unimplemented")
    }

    pub fn from_y_and_lsb_x (_y: BigInt, _lsb_x: bool) -> Self {
        todo!("Unimplemented")
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = rhs.x;
        let y2 = rhs.y;
        let num1 = (x1.clone() * y2.clone() + y1.clone() * x2.clone()) % bi!(P, 16);
        let denom1 = (bi!("1") + bi!(D, 16) * (x1.clone() * x2.clone() * y1.clone() * y2.clone())) % bi!(P, 16);
        let inv_denom1 = mod_inverse(denom1, bi!(P, 16));
        let num2 = (y1.clone() * y2.clone() - x1.clone() * x2.clone()) % bi!(P, 16);
        let denom2 = (bi!("1") - bi!(D, 16) * (x1.clone() * x2.clone() * y1.clone() * y2.clone())) % bi!(P, 16);
        let inv_denom2 = mod_inverse(denom2, bi!(P, 16));

        Point {
            x: (num1 * inv_denom1) % bi!(P, 16),
            y: (num2 * inv_denom2) % bi!(P, 16),
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        if self == Point::identity() {
            Point::identity()
        } else {
            Point::new(-self.x.clone(), self.y.clone())
        }
    }
}

impl Mul<Point> for BigInt {
    type Output = Point;

    fn mul(self, _rhs: Point) -> Point {
        todo!("Unimplemented")
    }
}

// extended gcd: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm a helpful utility function
pub fn extended_gcd(a: BigInt, b: BigInt) -> (BigInt, (BigInt, BigInt), (BigInt, BigInt)) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (BigInt::from(1), BigInt::from(0));
    let (mut old_t, mut t) = (BigInt::from(0), BigInt::from(1));
    
    while r != BigInt::zero() {
        let quotient = old_r.clone() / r.clone();
        let tmp = r.clone();
        (old_r, r) = (r, old_r - quotient.clone() * tmp);
        let tmp = s.clone();
        (old_s, s) = (s, old_s - quotient.clone() * tmp);
        let tmp = t.clone();
        (old_t, t) = (t, old_t - quotient.clone() * tmp);
    }
    (old_r, (old_s, old_t), (s, t))
}

pub fn mod_inverse(a: BigInt, m: BigInt) -> BigInt {
    let (gcd, (s, _), _) = extended_gcd(a.clone(), m.clone());
    if gcd != BigInt::from(1) {
        panic!("{} and {} are not coprime", a, m);
    }
    if s < BigInt::zero() {
        m + s
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let identity = Point::identity();
        assert_eq!(identity.x, BigInt::from(0));
        assert_eq!(identity.y, BigInt::from(1));
    }

    #[test]
    fn simple_egcd_test() {
        let (gcd, (s, t), (old_s, old_t)) = extended_gcd(bi!("240"), bi!("46"));
        assert_eq!([gcd, s, t, old_s, old_t],
            [bi!("2"), bi!("-9"), bi!("47"), bi!("23"), bi!("-120")]);
    }

    #[test]
    fn mod_inverse_test() {
        assert_eq!(mod_inverse(bi!("3"), bi!("11")), bi!("4"));
        assert_eq!(mod_inverse(bi!("5"), bi!("11")), bi!("9"));
        assert_eq!(mod_inverse(bi!("7"), bi!("11")), bi!("8"));
    }
}
