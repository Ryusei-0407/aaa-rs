use super::utils::modpow;
use primitive_types::U512;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct FieldElement {
    pub num: U512,
    pub prime: U512,
}

impl FieldElement {
    pub fn new(num: U512, prime: U512) -> FieldElement {
        if num >= prime || num < U512::zero() {
            panic!("Num {} not in field range 0 to {}", num, prime - 1)
        };

        FieldElement { num, prime }
    }

    pub fn div(&self, exp: U512) -> FieldElement {
        let (base, n) = (self.num, self.prime);
        let exp = n - exp - U512::one();

        FieldElement::new(modpow(base, exp, n), self.prime)
    }

    pub fn pow(&self, exp: U512) -> FieldElement {
        FieldElement::new(modpow(self.num, exp, self.prime), self.prime)
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            num: (self.num + other.num) % self.prime,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            num: (self.num + (self.prime - other.num)) % self.prime,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            num: (self.num * other.num) % self.prime,
            prime: self.prime,
        }
    }
}
