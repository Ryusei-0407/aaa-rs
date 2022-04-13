use super::field::Field;
use super::point::Point;
use primitive_types::U512;

static N: &str = "0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

pub fn secp256k1_new(x: &'static str, y: &'static str) -> Point {
    let prime =
        U512::from(2).pow(U512::from(256)) - U512::from(2).pow(U512::from(32)) - U512::from(977);
    let x = Field::new(U512::from(x), U512::from(prime));
    let y = Field::new(U512::from(y), U512::from(prime));
    let a: Field = Field::new(U512::zero(), U512::from(prime));
    let b: Field = Field::new(U512::from(7), U512::from(prime));

    Point::new(x, y, a, b)
}

pub fn secp256k1_g() -> Point {
    let prime =
        U512::from(2).pow(U512::from(256)) - U512::from(2).pow(U512::from(32)) - U512::from(977);
    let gx: &str = "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    let gy: &str = "0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
    let x = Field::new(U512::from(gx), U512::from(prime));
    let y = Field::new(U512::from(gy), U512::from(prime));
    let a: Field = Field::new(U512::zero(), U512::from(prime));
    let b: Field = Field::new(U512::from(7), U512::from(prime));

    Point::new(x, y, a, b)
}

pub fn secp256k1_zero() -> Point {
    let prime =
        U512::from(2).pow(U512::from(256)) - U512::from(2).pow(U512::from(32)) - U512::from(977);
    let a: Field = Field::new(U512::zero(), U512::from(prime));
    let b: Field = Field::new(U512::from(7), U512::from(prime));
    let zero = Field::new(U512::zero(), U512::from(prime));

    Point::new(zero, zero, a, b)
}
