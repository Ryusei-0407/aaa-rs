use num::bigint::{BigInt, ToBigInt};
use num::traits::{One, Zero};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq, Clone)]
pub struct FieldElement {
    pub num: BigInt,
    pub prime: BigInt,
}

impl FieldElement {
    pub fn new(num: BigInt, prime: BigInt) -> FieldElement {
        if num >= prime || num < Zero::zero() {
            panic!("Num {} not in field range 0 to {}", num, prime - 1)
        };

        FieldElement { num, prime }
    }

    pub fn div(&self, exp: BigInt) -> FieldElement {
        let (base, n) = (&self.num, &self.prime);
        let exp = exp + n - 1;

        FieldElement::new(base.modpow(&exp, n), self.prime.clone())
    }

    pub fn pow(&self, exp: BigInt) -> FieldElement {
        FieldElement::new(
            self.num
                .pow(exp.try_into().unwrap())
                .modpow(&1_i32.to_bigint().unwrap(), &self.prime),
            self.prime.clone(),
        )
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            num: (self.num + other.num).modpow(&One::one(), &self.prime),
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let prime = &self.prime.clone();
        Self {
            num: (self.num - other.num + prime).modpow(&One::one(), prime),
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            num: (self.num * other.num).modpow(&One::one(), &self.prime),
            prime: self.prime,
        }
    }
}
