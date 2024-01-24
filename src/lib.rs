use num_bigint::BigInt;
use num_traits::{Zero, One};
use std::{cmp::PartialEq, ops::{Add, Mul, Sub}};

const P: &str = "fffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
const R: &str = "3fffffffffffffffffffffffffffffffffffffffffffffffffffffff7cca23e9c44edb49aed63690216cc2728dc58f552378c292ab5844f3";
const D: &str = "-39081";
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
}