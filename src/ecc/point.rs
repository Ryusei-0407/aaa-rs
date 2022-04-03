use super::fieldelement::FieldElement;
use super::utils::to_binary;
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
        let (b, l) = to_binary(n);
        for i in 0..l {
            let (s, e) = (l - i - 1, l - i);
            if &b[s..e] == "1" {
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
        let p = self;
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
            let (y2, y1) = if self.y >= other.y {
                (self.y, other.y)
            } else {
                (other.y, self.y)
            };
            let (x2, x1) = if self.x >= other.x {
                (self.x, other.x)
            } else {
                (other.x, self.x)
            };
            (y2 - y1) * FieldElement::div(&(x2 - x1), U512::one())
        };

        let x = FieldElement::pow(&s, U512::from(2)) - self.x - other.x;
        let y = (s * (self.x - x)) - self.y;

        Self { x, y, a, b }
    }
}

#[test]
fn fieldelement_new() {
    let prime = U512::from(13);
    let a = FieldElement::new(U512::from(7), prime);
    let b = FieldElement::new(U512::from(6), prime);

    assert_eq!(a, a);
    assert_ne!(a, b);
}

#[test]
fn fieldelement_add() {
    let prime = U512::from(13);
    let a = FieldElement::new(U512::from(7), prime);
    let b = FieldElement::new(U512::from(12), prime);
    let c = FieldElement::new(U512::from(6), prime);

    assert_eq!((a + b), c);
}

#[test]
fn fieldelement_sub() {
    let prime = U512::from(13);
    let a = FieldElement::new(U512::from(7), prime);
    let b = FieldElement::new(U512::from(12), prime);
    let c = FieldElement::new(U512::from(5), prime);

    assert_eq!((b - a), c);
}

#[test]
fn fieldelement_mul() {
    let prime = U512::from(13);
    let a = FieldElement::new(U512::from(3), prime);
    let b = FieldElement::new(U512::from(12), prime);
    let c = FieldElement::new(U512::from(10), prime);

    assert_eq!((a * b), c);
}

#[test]
fn fieldelement_pow() {
    let prime = U512::from(13);
    let a = FieldElement::new(U512::from(3), prime);
    let b = FieldElement::new(U512::one(), prime);

    assert_eq!(FieldElement::pow(&a, U512::from(3)), b);
}

#[test]
fn fieldelement_div() {
    let prime = U512::from(13);
    let a = FieldElement::new(U512::from(7), prime);
    let b = FieldElement::new(U512::from(8), prime);

    assert_eq!(FieldElement::div(&a, U512::from(3)), b);
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

    let _p6 = Point::new(gx, gy, a, b);
}

#[test]
fn point_add1() {
    let prime = U512::from(223);
    let (a, b) = (
        FieldElement::new(U512::zero(), prime),
        FieldElement::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        FieldElement::new(U512::from(192), prime),
        FieldElement::new(U512::from(105), prime),
    );
    let (x2, y2) = (
        FieldElement::new(U512::from(17), prime),
        FieldElement::new(U512::from(56), prime),
    );

    let p1 = Point::new(x1, y1, a, b);
    let p2 = Point::new(x2, y2, a, b);

    let (x, y) = (
        FieldElement::new(U512::from(170), prime),
        FieldElement::new(U512::from(142), prime),
    );
    let ans = Point::new(x, y, a, b);

    assert_eq!(p1 + p2, ans);
}

#[test]
fn point_add2() {
    let prime = U512::from(223);
    let (a, b) = (
        FieldElement::new(U512::zero(), prime),
        FieldElement::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        FieldElement::new(U512::from(170), prime),
        FieldElement::new(U512::from(142), prime),
    );
    let (x2, y2) = (
        FieldElement::new(U512::from(60), prime),
        FieldElement::new(U512::from(139), prime),
    );

    let p1 = Point::new(x1, y1, a, b);
    let p2 = Point::new(x2, y2, a, b);

    let (x, y) = (
        FieldElement::new(U512::from(220), prime),
        FieldElement::new(U512::from(181), prime),
    );
    let ans = Point::new(x, y, a, b);

    assert_eq!(p1 + p2, ans);
}

#[test]
fn point_add3() {
    let prime = U512::from(223);
    let (a, b) = (
        FieldElement::new(U512::zero(), prime),
        FieldElement::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        FieldElement::new(U512::from(47), prime),
        FieldElement::new(U512::from(71), prime),
    );
    let (x2, y2) = (
        FieldElement::new(U512::from(17), prime),
        FieldElement::new(U512::from(56), prime),
    );

    let p1 = Point::new(x1, y1, a, b);
    let p2 = Point::new(x2, y2, a, b);

    let (x, y) = (
        FieldElement::new(U512::from(215), prime),
        FieldElement::new(U512::from(68), prime),
    );
    let ans = Point::new(x, y, a, b);

    assert_eq!(p1 + p2, ans);
}

#[test]
fn point_add4() {
    let prime = U512::from(223);
    let (a, b) = (
        FieldElement::new(U512::zero(), prime),
        FieldElement::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        FieldElement::new(U512::from(143), prime),
        FieldElement::new(U512::from(98), prime),
    );
    let (x2, y2) = (
        FieldElement::new(U512::from(76), prime),
        FieldElement::new(U512::from(66), prime),
    );

    let p1 = Point::new(x1, y1, a, b);
    let p2 = Point::new(x2, y2, a, b);

    let (x, y) = (
        FieldElement::new(U512::from(47), prime),
        FieldElement::new(U512::from(71), prime),
    );
    let ans = Point::new(x, y, a, b);

    assert_eq!(p1 + p2, ans);
}

#[test]
fn point_mul1() {
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
        FieldElement::new(U512::from(47), prime),
        FieldElement::new(U512::from(71), prime),
    );

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, U512::one()), ans);
}

#[test]
fn point_mul2() {
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
        FieldElement::new(U512::from(36), prime),
        FieldElement::new(U512::from(111), prime),
    );

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, U512::from(2)), ans);
}

/*
#[test]
fn point_mul3() {
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
        FieldElement::new(U512::from(15), prime),
        FieldElement::new(U512::from(137), prime),
    );

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, U512::from(3)), ans);
}

#[test]
fn point_mul4() {
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
        FieldElement::new(U512::from(194), prime),
        FieldElement::new(U512::from(51), prime),
    );

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, U512::from(4)), ans);
}

#[test]
fn point_mul5() {
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
        FieldElement::new(U512::from(126), prime),
        FieldElement::new(U512::from(96), prime),
    );

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, U512::from(5)), ans);
}

#[test]
fn point_mul6() {
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
        FieldElement::new(U512::from(139), prime),
        FieldElement::new(U512::from(137), prime),
    );

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, U512::from(6)), ans);
}

#[test]
fn point_mul7() {
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
        FieldElement::new(U512::from(92), prime),
        FieldElement::new(U512::from(47), prime),
    );

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, U512::from(7)), ans);
}

#[test]
fn point_mul8() {
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
        FieldElement::new(U512::from(116), prime),
        FieldElement::new(U512::from(55), prime),
    );

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, U512::from(8)), ans);
}

#[test]
fn point_mul9() {
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
        FieldElement::new(U512::from(69), prime),
        FieldElement::new(U512::from(86), prime),
    );

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, U512::from(9)), ans);
}

#[test]
fn point_mul10() {
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
*/
