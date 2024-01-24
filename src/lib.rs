use num_bigint::BigInt;
use num_traits::{Zero, One};
use std::{cmp::PartialEq, ops::{Add, Mul, Sub}};

pub struct Point {
    x: BigInt,
    y: BigInt,
}

impl Point {
    pub fn zero() -> Self {
        x: BigInt::from(0),
        y: BigInt::from(1),
    }

    pub fn new(x: BigInt, y: BigInt) -> Self {
        x,
        y,
    }
}

impl Add<Point> for Point {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
