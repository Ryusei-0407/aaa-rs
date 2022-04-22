use super::field::Field;
use primitive_types::U512;
use std::ops::{Add, AddAssign};
use std::sync::Mutex;
use std::thread;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: Field,
    pub y: Field,
    pub a: Field,
    pub b: Field,
}

impl Point {
    pub fn new(x: Field, y: Field, a: Field, b: Field) -> Point {
        if x.num.is_zero() && y.num.is_zero() {
            return Point { x, y, a, b };
        }

        if Field::pow(&y, U512::from(2)) != Field::pow(&x, U512::from(3)) + (a * x) + b {
            panic!("({:?}, {:?}) is not on the curve", x, y)
        }

        Point { x, y, a, b }
    }

    pub fn mul(&self, n: U512) -> Point {
        let mut ans = Point::new(
            Field::new(U512::zero(), self.a.prime),
            Field::new(U512::zero(), self.a.prime),
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
        let ans = Mutex::new(Point::new(
            Field::new(U512::zero(), self.a.prime),
            Field::new(U512::zero(), self.a.prime),
            self.a,
            self.b,
        ));
        let p = self.clone();
        let n = k % 16;
        let k = k / 16;
        if n != 0 {
            let mut ans = ans.lock().unwrap();
            for _ in 0..n {
                *ans += p;
            }
        }
        thread::scope(|s| {
            for _ in 0..16 {
                s.spawn(|| {
                    let mut ans = ans.lock().unwrap();
                    for _ in 0..k {
                        *ans += p;
                    }
                });
            }
        });

        let result = *ans.lock().unwrap();
        result
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
                x: Field::new(U512::zero(), prime),
                y: Field::new(U512::zero(), prime),
                a,
                b,
            };
        };

        if self.x == other.x && self.y != other.y {
            return Self {
                x: Field::new(U512::zero(), prime),
                y: Field::new(U512::zero(), prime),
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
            ((Field::pow(&self.x, U512::from(2)) * Field::new(U512::from(3), prime)) + a)
                * (Field::div(&(self.y * Field::new(U512::from(2), prime)), U512::one()))
        } else {
            let (x1, y1) = (self.x, self.y);
            let (x2, y2) = (other.x, other.y);
            (y2 - y1) * Field::div(&(x2 - x1), U512::one())
        };

        let x = Field::pow(&s, U512::from(2)) - self.x - other.x;
        let y = (s * (self.x - x)) - self.y;

        Self { x, y, a, b }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        let ans = *self + other;
        *self = Self {
            x: ans.x,
            y: ans.y,
            a: ans.a,
            b: ans.b,
        };
    }
}

#[test]
fn point_new() {
    let prime =
        U512::from(2).pow(U512::from(256)) - U512::from(2).pow(U512::from(32)) - U512::from(977);
    let (a, b) = (
        Field::new(U512::zero(), prime),
        Field::new(U512::from(7), prime),
    );
    let x = U512::from("0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
    let y = U512::from("0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8");
    let (gx, gy) = (Field::new(x, prime), Field::new(y, prime));

    let _p = Point::new(gx, gy, a, b);
}

#[test]
fn point_mul1() {
    let prime = U512::from(223);
    let (a, b) = (
        Field::new(U512::zero(), prime),
        Field::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        Field::new(U512::from(47), prime),
        Field::new(U512::from(71), prime),
    );

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (
        Field::new(U512::from(47), prime),
        Field::new(U512::from(71), prime),
    );

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, U512::one()), ans);
}

#[test]
fn point_mul2() {
    let prime = U512::from(223);
    let (a, b) = (
        Field::new(U512::zero(), prime),
        Field::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        Field::new(U512::from(47), prime),
        Field::new(U512::from(71), prime),
    );

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (
        Field::new(U512::from(36), prime),
        Field::new(U512::from(111), prime),
    );

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, U512::from(2)), ans);
}

#[test]
fn point_mul3() {
    let prime = U512::from(223);
    let (a, b) = (
        Field::new(U512::zero(), prime),
        Field::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        Field::new(U512::from(47), prime),
        Field::new(U512::from(71), prime),
    );
    let p = Point::new(x1, y1, a, b);
    let (x, y) = (
        Field::new(U512::from(15), prime),
        Field::new(U512::from(137), prime),
    );
    let ans = Point::new(x, y, a, b);
    assert_eq!(Point::mul(&p, U512::from(3)), ans);
}

#[test]
fn point_mul4() {
    let prime = U512::from(223);
    let (a, b) = (
        Field::new(U512::zero(), prime),
        Field::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        Field::new(U512::from(47), prime),
        Field::new(U512::from(71), prime),
    );
    let p = Point::new(x1, y1, a, b);
    let (x, y) = (
        Field::new(U512::from(194), prime),
        Field::new(U512::from(51), prime),
    );
    let ans = Point::new(x, y, a, b);
    assert_eq!(Point::mul(&p, U512::from(4)), ans);
}

#[test]
fn point_mul5() {
    let prime = U512::from(223);
    let (a, b) = (
        Field::new(U512::zero(), prime),
        Field::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        Field::new(U512::from(47), prime),
        Field::new(U512::from(71), prime),
    );
    let p = Point::new(x1, y1, a, b);
    let (x, y) = (
        Field::new(U512::from(126), prime),
        Field::new(U512::from(96), prime),
    );
    let ans = Point::new(x, y, a, b);
    assert_eq!(Point::mul(&p, U512::from(5)), ans);
}

#[test]
fn point_mul6() {
    let prime = U512::from(223);
    let (a, b) = (
        Field::new(U512::zero(), prime),
        Field::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        Field::new(U512::from(47), prime),
        Field::new(U512::from(71), prime),
    );
    let p = Point::new(x1, y1, a, b);
    let (x, y) = (
        Field::new(U512::from(139), prime),
        Field::new(U512::from(137), prime),
    );
    let ans = Point::new(x, y, a, b);
    assert_eq!(Point::mul(&p, U512::from(6)), ans);
}

#[test]
fn point_mul7() {
    let prime = U512::from(223);
    let (a, b) = (
        Field::new(U512::zero(), prime),
        Field::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        Field::new(U512::from(47), prime),
        Field::new(U512::from(71), prime),
    );
    let p = Point::new(x1, y1, a, b);
    let (x, y) = (
        Field::new(U512::from(92), prime),
        Field::new(U512::from(47), prime),
    );
    let ans = Point::new(x, y, a, b);
    assert_eq!(Point::mul(&p, U512::from(7)), ans);
}

#[test]
fn point_mul8() {
    let prime = U512::from(223);
    let (a, b) = (
        Field::new(U512::zero(), prime),
        Field::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        Field::new(U512::from(47), prime),
        Field::new(U512::from(71), prime),
    );
    let p = Point::new(x1, y1, a, b);
    let (x, y) = (
        Field::new(U512::from(116), prime),
        Field::new(U512::from(55), prime),
    );
    let ans = Point::new(x, y, a, b);
    assert_eq!(Point::mul(&p, U512::from(8)), ans);
}

#[test]
fn point_mul9() {
    let prime = U512::from(223);
    let (a, b) = (
        Field::new(U512::zero(), prime),
        Field::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        Field::new(U512::from(47), prime),
        Field::new(U512::from(71), prime),
    );
    let p = Point::new(x1, y1, a, b);
    let (x, y) = (
        Field::new(U512::from(69), prime),
        Field::new(U512::from(86), prime),
    );
    let ans = Point::new(x, y, a, b);
    assert_eq!(Point::mul(&p, U512::from(9)), ans);
}

#[test]
fn point_mul10() {
    let prime = U512::from(223);
    let (a, b) = (
        Field::new(U512::zero(), prime),
        Field::new(U512::from(7), prime),
    );
    let (x1, y1) = (
        Field::new(U512::from(47), prime),
        Field::new(U512::from(71), prime),
    );

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (
        Field::new(U512::from(154), prime),
        Field::new(U512::from(150), prime),
    );

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, U512::from(10)), ans);
}
