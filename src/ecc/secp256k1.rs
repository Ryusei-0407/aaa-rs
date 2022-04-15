use super::field::Field;
use super::point::Point;
use primitive_types::U512;

static N: &str = "0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

pub fn secp256k1_new(x: &'static str, y: &'static str) -> Point {
    let prime =
        U512::from(2).pow(U512::from(256)) - U512::from(2).pow(U512::from(32)) - U512::from(977);
    let x = Field::new(U512::from(x), prime);
    let y = Field::new(U512::from(y), prime);
    let a = Field::new(U512::zero(), prime);
    let b = Field::new(U512::from(7), prime);

    Point::new(x, y, a, b)
}

pub fn secp256k1_g() -> Point {
    let prime =
        U512::from(2).pow(U512::from(256)) - U512::from(2).pow(U512::from(32)) - U512::from(977);
    let gx = "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    let gy = "0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
    let x = Field::new(U512::from(gx), prime);
    let y = Field::new(U512::from(gy), prime);
    let a = Field::new(U512::zero(), prime);
    let b = Field::new(U512::from(7), prime);

    Point::new(x, y, a, b)
}

pub fn secp256k1_zero() -> Point {
    let prime =
        U512::from(2).pow(U512::from(256)) - U512::from(2).pow(U512::from(32)) - U512::from(977);
    let a = Field::new(U512::zero(), prime);
    let b = Field::new(U512::from(7), prime);
    let zero = Field::new(U512::zero(), prime);

    Point::new(zero, zero, a, b)
}

/*
#[test]
fn demo() {
    use super::utils::modpow;
    let z = U512::from("0xbc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423");
    let r = U512::from("0x37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6");
    let s = U512::from("0x8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
    let px = "0x04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574";
    let py = "0x82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4";

    let n = U512::from(N);

    let p = secp256k1_new(px, py);
    let g = secp256k1_g();
    let s_inv: U512 = modpow(s, n - U512::from(2), n);
    let u: U512 = z * s_inv % n;
    let v: U512 = r * s_inv % n;

    assert_eq!((Point::mul(&g, u) + Point::mul(&p, v)).x.num, r);
}
*/
