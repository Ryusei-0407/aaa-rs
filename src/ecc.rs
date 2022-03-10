#[derive(Debug, PartialEq)]
pub struct FieldElement {
    num: isize,
    prime: isize,
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
        FieldElement::new(
            self.num
                .pow((exponent + self.prime - 1).try_into().unwrap())
                % self.prime,
            self.prime,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Point {
    x: isize,
    y: isize,
    a: isize,
    b: isize,
}

impl Point {
    pub fn new(x: isize, y: isize, a: isize, b: isize) -> Point {
        if y.pow(2) != x.pow(3) + a * x + b {
            panic!("({x}, {y}) is not on the curve")
        };

        Point { x, y, a, b }
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

#[test]
fn point_new() {
    let p1 = Point::new(-1, -1, 5, 7);
    let p2 = Point::new(18, 77, 5, 7);

    assert_eq!(p1, p1);
    assert_ne!(p1, p2);
}

#[test]
#[should_panic]
fn point_new_panic() {
    let _p2 = Point::new(-1, -2, 5, 7);
}
