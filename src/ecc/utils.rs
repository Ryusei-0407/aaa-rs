use primitive_types::U512;

pub fn modpow(base: U512, exp: U512, n: U512) -> U512 {
    let (mut b, mut exp) = (base, exp);

    if exp.is_zero() {
        return U512::one();
    }

    let mut res = U512::one();
    b %= n;

    loop {
        if exp % 2 == U512::one() {
            res *= b;
            res %= n;
        }

        if exp == U512::one() {
            return res;
        }

        exp /= 2;
        b *= b;
        b %= n;
    }
}
