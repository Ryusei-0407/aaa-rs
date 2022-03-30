#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FieldElement {
    pub num: isize,
    pub prime: isize,
}

fn modpow(base: &isize, exp: &isize, n: &isize) -> isize {
    let (mut base, mut exp, n) = (*base, *exp, *n);
    let n = n.abs();

    if exp == 0 {
        return 1;
    }

    let mut res: isize = 1;
    base %= &n;

    loop {
        if &exp % 2 == 1 {
            res *= &base;
            res %= &n;
        }

        if exp == 1 {
            return res;
        }

        exp /= 2;
        base *= base.clone();
        base %= &n;
    }
}

impl FieldElement {
    pub fn new(num: isize, prime: isize) -> FieldElement {
        if num >= prime || num < 0 {
            panic!("Num {} not in field range 0 to {}", num, prime - 1)
        };

        FieldElement { num, prime }
    }

    pub fn add(&self, other: &FieldElement) -> FieldElement {
        FieldElement::new((self.num + other.num) % self.prime, self.prime)
    }

    pub fn sub(&self, other: &FieldElement) -> FieldElement {
        FieldElement::new((self.num - other.num + self.prime) % self.prime, self.prime)
    }

    pub fn mul(&self, other: &FieldElement) -> FieldElement {
        FieldElement::new((self.num * other.num) % self.prime, self.prime)
    }

    pub fn pow(&self, exponent: isize) -> FieldElement {
        FieldElement::new(
            self.num.pow(exponent.try_into().unwrap()) % self.prime,
            self.prime,
        )
    }

    pub fn div(&self, exponent: isize) -> FieldElement {
        let (base, n) = (&self.num, &self.prime);
        let exp = exponent + n - 1;
        FieldElement::new(modpow(&base, &exp, &n), self.prime)
    }
}

#[test]
fn ecc_new() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(6, 13);

    assert_eq!(a, a);
    assert_ne!(a, b);
}

#[test]
fn ecc_add() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(12, 13);
    let c = FieldElement::new(6, 13);

    assert_eq!(FieldElement::add(&a, &b), c);
}

#[test]
fn ecc_sub() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(12, 13);
    let c = FieldElement::new(5, 13);

    assert_eq!(FieldElement::sub(&b, &a), c);
}

#[test]
fn ecc_mul() {
    let a = FieldElement::new(3, 13);
    let b = FieldElement::new(12, 13);
    let c = FieldElement::new(10, 13);

    assert_eq!(FieldElement::mul(&a, &b), c);
}

#[test]
fn ecc_pow() {
    let a = FieldElement::new(3, 13);
    let b = FieldElement::new(1, 13);

    assert_eq!(FieldElement::pow(&a, 3), b);
}

#[test]
fn ecc_div() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(8, 13);

    assert_eq!(FieldElement::div(&a, -3), b);
}
