use super::fieldelement::FieldElement;
use primitive_types::U512;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    x: FieldElement,
    y: FieldElement,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    pub fn new(x: FieldElement, y: FieldElement, a: FieldElement, b: FieldElement) -> Point {
        if x.num.is_zero() && y.num.is_zero() {
            return Point { x, y, a, b };
        }

        if FieldElement::pow(&y, U512::from(2))
            != FieldElement::pow(&x, U512::from(3)) + (a * x) + b
        {
            panic!("({:?}, {:?}) is not on the curve", x, y)
        }

        Point { x, y, a, b }
    }

    pub fn mul(&self, n: U512) -> Point {
        let mut ans = Point::new(
            FieldElement::new(U512::zero(), self.a.prime),
            FieldElement::new(U512::zero(), self.a.prime),
            self.a,
            self.b,
        );
        let p = self.clone();
        let l = n.bits();
        for i in 0..l {
            if n.bit(i) {
                let q = Point::scalar(&p, 2_usize.pow(i.try_into().unwrap()));
                ans = ans + q;
            }
        }
        ans
    }

    fn scalar(&self, k: usize) -> Point {
        let mut ans = Point::new(
            FieldElement::new(U512::zero(), self.a.prime),
            FieldElement::new(U512::zero(), self.a.prime),
            self.a,
            self.b,
        );
        let p = self.clone();
        for _i in 0..k {
            ans = ans + p.clone();
        }
        ans
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a.num != other.a.num || self.b.num != other.b.num {
            panic!("Points {:?}, {:?} are not on the same curve", self, other)
        };
        let a = self.a;
        let b = self.b;
        let prime = a.prime;

        if self == other && self.y.num.is_zero() {
            return Self {
                x: FieldElement::new(U512::zero(), prime),
                y: FieldElement::new(U512::zero(), prime),
                a,
                b,
            };
        };

        if self.x == other.x && self.y != other.y {
            return Self {
                x: FieldElement::new(U512::zero(), prime),
                y: FieldElement::new(U512::zero(), prime),
                a,
                b,
            };
        };

        if self.x.num.is_zero() {
            return Self {
                x: other.x,
                y: other.y,
                a,
                b,
            };
        };

        if other.x.num.is_zero() {
            return Self {
                x: self.x,
                y: self.y,
                a,
                b,
            };
        };

        let s = if self == other {
            ((FieldElement::pow(&self.x, U512::from(2)) * FieldElement::new(U512::from(3), prime))
                + a)
                * (FieldElement::div(
                    &(self.y * FieldElement::new(U512::from(2), prime)),
                    U512::one(),
                ))
        } else {
            let (x1, y1) = (self.x, self.y);
            let (x2, y2) = (other.x, other.y);
            (y2 - y1) * FieldElement::div(&(x2 - x1), U512::one())
        };

        let x = FieldElement::pow(&s, U512::from(2)) - self.x - other.x;
        let y = (s * (self.x - x)) - self.y;

        Self { x, y, a, b }
    }
}

#[test]
fn point_new() {
    let prime =
        U512::from(2).pow(U512::from(256)) - U512::from(2).pow(U512::from(32)) - U512::from(977);
    let (a, b) = (
        FieldElement::new(U512::zero(), prime),
        FieldElement::new(U512::from(7), prime),
    );
    let x = U512::from("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
    let y = U512::from("0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8");
    let (gx, gy) = (FieldElement::new(x, prime), FieldElement::new(y, prime));

    let _p = Point::new(gx, gy, a, b);
}

#[test]
fn point_mul() {
    let prime = U512::from(223);
    let (a, b) = (
        FieldElement::new(U512::zero(), prime),
        FieldElement::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        FieldElement::new(U512::from(47), prime),
        FieldElement::new(U512::from(71), prime),
    );

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (
        FieldElement::new(U512::from(154), prime),
        FieldElement::new(U512::from(150), prime),
    );

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, U512::from(10)), ans);
}
