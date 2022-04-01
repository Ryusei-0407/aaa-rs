mod fieldelement;

use fieldelement::*;
use num::bigint::ToBigInt;
use num::traits::Zero;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    x: FieldElement,
    y: FieldElement,
    a: FieldElement,
    b: FieldElement,
}

/*
fn to_binary(n: BigInt) -> (String, BigInt) {
    let mut n = n;
    let mut b = String::from("");
    while n >= Zero::zero() {
        if n % 2 == Zero::zero() {
            b.push('0')
        } else {
            b.push('1')
        }
        n = n / 2;
    }
    b = b.chars().rev().collect();

    let l = b.len();

    (b, l)
}
*/

impl Point {
    pub fn new(x: FieldElement, y: FieldElement, a: FieldElement, b: FieldElement) -> Point {
        if x.num == Zero::zero() && y.num == Zero::zero() {
            Point { x, y, a, b }
        } else if a.num == Zero::zero()
            && FieldElement::pow(&y, 2_i32.to_bigint().unwrap())
                != FieldElement::pow(&x, 3_i32.to_bigint().unwrap()) + b.clone()
        {
            panic!("({:?}, {:?}) is not on the curve", x, y)
        } else if FieldElement::pow(&y, 2_i32.to_bigint().unwrap())
            != FieldElement::pow(&x, 3_i32.to_bigint().unwrap()) + a.clone() * x.clone() + b.clone()
        {
            panic!("({:?}, {:?}) is not on the curve", x, y)
        } else {
            Point { x, y, a, b }
        }
    }

    /*
    pub fn add(&self, other: &Point) -> Point {
        if self.a.num != other.a.num || self.b.num != other.b.num {
            panic!("Points {:?}, {:?} are not on the same curve", self, other)
        };
        let a = &self.a;
        let b = &self.b;
        let prime = &a.prime;

        if self == other && self.y.num == 0 {
            return Point::new(
                FieldElement::new(0, *prime),
                FieldElement::new(0, *prime),
                *a,
                *b,
            );
        };

        if self.x == other.x && self.y != other.y {
            return Point::new(
                FieldElement::new(0, *prime),
                FieldElement::new(0, *prime),
                *a,
                *b,
            );
        };

        let x = (self.x.num, other.x.num);
        match x {
            (0, ..) => {
                let (x, y) = (&other.x, &other.y);
                Point::new(*x, *y, *a, *b)
            }
            (.., 0) => Point::new(self.x, self.y, *a, *b),
            _ => {
                let s = if self.x != other.x {
                    FieldElement::mul(
                        &FieldElement::sub(&other.y, &self.y),
                        &FieldElement::div(&FieldElement::sub(&other.x, &self.x), -1),
                    )
                } else if self.x == other.x && self.y == other.y {
                    FieldElement::mul(
                        &FieldElement::add(
                            &FieldElement::mul(
                                &FieldElement::pow(&self.x, 2),
                                &FieldElement::new(3, *prime),
                            ),
                            a,
                        ),
                        &FieldElement::div(
                            &FieldElement::mul(&self.y, &FieldElement::new(2, *prime)),
                            -1,
                        ),
                    )
                } else {
                    FieldElement::mul(
                        &FieldElement::sub(&other.y, &self.y),
                        &FieldElement::div(&FieldElement::sub(&other.x, &self.x), -1),
                    )
                };
                let x = FieldElement::sub(
                    &FieldElement::sub(&FieldElement::pow(&s, 2), &self.x),
                    &other.x,
                );
                let y = FieldElement::sub(
                    &FieldElement::mul(&s, &FieldElement::sub(&self.x, &x)),
                    &self.y,
                );

                Point::new(x, y, *a, *b)
            }
        }
    }

    pub fn mul(&self, n: usize) -> Point {
        let mut ans = Point::new(
            FieldElement::new(0, self.a.prime),
            FieldElement::new(0, self.a.prime),
            self.a,
            self.b,
        );
        let p = self.clone();
        let (b, l) = to_binary(n);
        for i in 0..l {
            let (s, e) = (l - i - 1, l - i);
            if &b[s..e] == "1" {
                let q = Point::scalar(&p, 2_usize.pow(i.try_into().unwrap()));
                ans = Point::add(&ans, &q);
            }
        }
        ans
    }

    fn scalar(&self, k: usize) -> Point {
        let mut ans = Point::new(
            FieldElement::new(0, self.a.prime),
            FieldElement::new(0, self.a.prime),
            self.a,
            self.b,
        );
        let p = self.clone();
        for _i in 0..k {
            ans = Point::add(&ans, &p);
        }
        ans
    }
    */
}

#[test]
fn fieldelement_new() {
    let prime = 13_i32.to_bigint().unwrap();
    let a = FieldElement::new(7_i32.to_bigint().unwrap(), prime.clone());
    let b = FieldElement::new(6_i32.to_bigint().unwrap(), prime.clone());

    assert_eq!(a, a);
    assert_ne!(a, b);
}

#[test]
fn fieldelement_add() {
    let prime = 13_i32.to_bigint().unwrap();
    let a = FieldElement::new(7_i32.to_bigint().unwrap(), prime.clone());
    let b = FieldElement::new(12_i32.to_bigint().unwrap(), prime.clone());
    let c = FieldElement::new(6_i32.to_bigint().unwrap(), prime.clone());

    assert_eq!((a + b), c);
}

#[test]
fn fieldelement_sub() {
    let prime = 13_i32.to_bigint().unwrap();
    let a = FieldElement::new(7_i32.to_bigint().unwrap(), prime.clone());
    let b = FieldElement::new(12_i32.to_bigint().unwrap(), prime.clone());
    let c = FieldElement::new(5_i32.to_bigint().unwrap(), prime.clone());

    assert_eq!((b - a), c);
}

#[test]
fn fieldelement_mul() {
    let prime = 13_i32.to_bigint().unwrap();
    let a = FieldElement::new(3_i32.to_bigint().unwrap(), prime.clone());
    let b = FieldElement::new(12_i32.to_bigint().unwrap(), prime.clone());
    let c = FieldElement::new(10_i32.to_bigint().unwrap(), prime.clone());

    assert_eq!((a * b), c);
}

#[test]
fn fieldelement_pow() {
    let prime = 13_i32.to_bigint().unwrap();
    let a = FieldElement::new(3_i32.to_bigint().unwrap(), prime.clone());
    let b = FieldElement::new(1_i32.to_bigint().unwrap(), prime.clone());

    assert_eq!(FieldElement::pow(&a, 3_i32.to_bigint().unwrap()), b);
}

#[test]
fn fieldelement_div() {
    let prime = 13_i32.to_bigint().unwrap();
    let a = FieldElement::new(7_i32.to_bigint().unwrap(), prime.clone());
    let b = FieldElement::new(8_i32.to_bigint().unwrap(), prime.clone());

    assert_eq!(FieldElement::div(&a, -3_i32.to_bigint().unwrap()), b);
}

#[test]
fn point_new1() {
    let prime = 223_i32.to_bigint().unwrap();
    let (a, b) = (
        FieldElement::new(Zero::zero(), prime.clone()),
        FieldElement::new(7_i32.to_bigint().unwrap(), prime.clone()),
    );
    let (x, y) = (
        FieldElement::new(192_i32.to_bigint().unwrap(), prime.clone()),
        FieldElement::new(105_i32.to_bigint().unwrap(), prime.clone()),
    );

    let _p1 = Point::new(x, y, a, b);
}

#[test]
fn point_new2() {
    let prime = 223_i32.to_bigint().unwrap();
    let (a, b) = (
        FieldElement::new(Zero::zero(), prime.clone()),
        FieldElement::new(7_i32.to_bigint().unwrap(), prime.clone()),
    );
    let (x, y) = (
        FieldElement::new(17_i32.to_bigint().unwrap(), prime.clone()),
        FieldElement::new(56_i32.to_bigint().unwrap(), prime.clone()),
    );

    let _p2 = Point::new(x, y, a, b);
}

#[test]
#[should_panic]
fn point_new3() {
    let prime = 223_i32.to_bigint().unwrap();
    let (a, b) = (
        FieldElement::new(Zero::zero(), prime.clone()),
        FieldElement::new(7_i32.to_bigint().unwrap(), prime.clone()),
    );
    let (x, y) = (
        FieldElement::new(200_i32.to_bigint().unwrap(), prime.clone()),
        FieldElement::new(119_i32.to_bigint().unwrap(), prime.clone()),
    );

    let _p3 = Point::new(x, y, a, b);
}

#[test]
fn point_new4() {
    let prime = 223_i32.to_bigint().unwrap();
    let (a, b) = (
        FieldElement::new(Zero::zero(), prime.clone()),
        FieldElement::new(7_i32.to_bigint().unwrap(), prime.clone()),
    );
    let (x, y) = (
        FieldElement::new(num::traits::One::one(), prime.clone()),
        FieldElement::new(193_i32.to_bigint().unwrap(), prime.clone()),
    );

    let _p4 = Point::new(x, y, a, b);
}

#[test]
#[should_panic]
fn point_new5() {
    let prime = 223_i32.to_bigint().unwrap();
    let (a, b) = (
        FieldElement::new(Zero::zero(), prime.clone()),
        FieldElement::new(7_i32.to_bigint().unwrap(), prime.clone()),
    );
    let (x, y) = (
        FieldElement::new(42_i32.to_bigint().unwrap(), prime.clone()),
        FieldElement::new(99_i32.to_bigint().unwrap(), prime.clone()),
    );

    let _p5 = Point::new(x, y, a, b);
}

/*
#[test]
fn point_new6() {
    let prime = 2_isize.pow(256) - 2_isize.pow(32) - 997;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let x = 0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798isize;
    let y = 0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8isize;
    let (gx, gy) = (FieldElement::new(x, prime), FieldElement::new(y, prime));

    let _p6 = Point::new(gx, gy, a, b);
}
*/

/*
#[test]
fn point_add1() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(192, prime), FieldElement::new(105, prime));
    let (x2, y2) = (FieldElement::new(17, prime), FieldElement::new(56, prime));

    let p1 = Point::new(x1, y1, a, b);
    let p2 = Point::new(x2, y2, a, b);

    let (x, y) = (FieldElement::new(170, prime), FieldElement::new(142, prime));
    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::add(&p1, &p2), ans);
}

#[test]
fn point_add2() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(170, prime), FieldElement::new(142, prime));
    let (x2, y2) = (FieldElement::new(60, prime), FieldElement::new(139, prime));

    let p1 = Point::new(x1, y1, a, b);
    let p2 = Point::new(x2, y2, a, b);

    let (x, y) = (FieldElement::new(220, prime), FieldElement::new(181, prime));
    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::add(&p1, &p2), ans);
}

#[test]
fn point_add3() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(47, prime), FieldElement::new(71, prime));
    let (x2, y2) = (FieldElement::new(17, prime), FieldElement::new(56, prime));

    let p1 = Point::new(x1, y1, a, b);
    let p2 = Point::new(x2, y2, a, b);

    let (x, y) = (FieldElement::new(215, prime), FieldElement::new(68, prime));
    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::add(&p1, &p2), ans);
}

#[test]
fn point_add4() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(143, prime), FieldElement::new(98, prime));
    let (x2, y2) = (FieldElement::new(76, prime), FieldElement::new(66, prime));

    let p1 = Point::new(x1, y1, a, b);
    let p2 = Point::new(x2, y2, a, b);

    let (x, y) = (FieldElement::new(47, prime), FieldElement::new(71, prime));
    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::add(&p1, &p2), ans);
}

#[test]
fn point_mul1() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(47, prime), FieldElement::new(71, prime));

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (FieldElement::new(47, prime), FieldElement::new(71, prime));

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, 1), ans);
}

#[test]
fn point_mul2() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(47, prime), FieldElement::new(71, prime));

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (FieldElement::new(36, prime), FieldElement::new(111, prime));

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, 2), ans);
}

#[test]
fn point_mul3() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(47, prime), FieldElement::new(71, prime));

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (FieldElement::new(15, prime), FieldElement::new(137, prime));

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, 3), ans);
}

#[test]
fn point_mul4() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(47, prime), FieldElement::new(71, prime));

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (FieldElement::new(194, prime), FieldElement::new(51, prime));

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, 4), ans);
}

#[test]
fn point_mul5() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(47, prime), FieldElement::new(71, prime));

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (FieldElement::new(126, prime), FieldElement::new(96, prime));

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, 5), ans);
}

#[test]
fn point_mul6() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(47, prime), FieldElement::new(71, prime));

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (FieldElement::new(139, prime), FieldElement::new(137, prime));

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, 6), ans);
}

#[test]
fn point_mul7() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(47, prime), FieldElement::new(71, prime));

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (FieldElement::new(92, prime), FieldElement::new(47, prime));

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, 7), ans);
}

#[test]
fn point_mul8() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(47, prime), FieldElement::new(71, prime));

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (FieldElement::new(116, prime), FieldElement::new(55, prime));

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, 8), ans);
}

#[test]
fn point_mul9() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(47, prime), FieldElement::new(71, prime));

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (FieldElement::new(69, prime), FieldElement::new(86, prime));

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, 9), ans);
}

#[test]
fn point_mul10() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x1, y1) = (FieldElement::new(47, prime), FieldElement::new(71, prime));

    let p = Point::new(x1, y1, a, b);

    let (x, y) = (FieldElement::new(154, prime), FieldElement::new(150, prime));

    let ans = Point::new(x, y, a, b);

    assert_eq!(Point::mul(&p, 10), ans);
}
*/
