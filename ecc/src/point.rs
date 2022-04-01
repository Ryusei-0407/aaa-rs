mod fieldelement;

use fieldelement::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    x: FieldElement,
    y: FieldElement,
    a: FieldElement,
    b: FieldElement,
}

fn to_binary(n: usize) -> (String, usize) {
    let mut n = n;
    let mut b = String::from("");
    while n >= 1 {
        if n % 2 == 0 {
            b.push('0')
        } else {
            b.push('1')
        }
        n /= 2;
    }
    b = b.chars().rev().collect();

    let l = b.len();

    (b, l)
}

impl Point {
    pub fn new(x: FieldElement, y: FieldElement, a: FieldElement, b: FieldElement) -> Point {
        if x.num == 0 && y.num == 0 {
            return Point { x, y, a, b };
        } else if a.num == 0
            && FieldElement::pow(&y, 2) != FieldElement::add(&FieldElement::pow(&x, 3), &b)
        {
            panic!("({:?}, {:?}) is not on the curve", x, y,)
        } else if FieldElement::pow(&y, 2)
            != FieldElement::add(
                &FieldElement::add(&FieldElement::pow(&x, 3), &FieldElement::mul(&a, &x)),
                &b,
            )
        {
            panic!("({:?}, {:?}) is not on the curve", x, y)
        } else {
            return Point { x, y, a, b };
        }
    }

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
}

#[test]
fn point_new1() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x, y) = (FieldElement::new(192, prime), FieldElement::new(105, prime));

    let _p1 = Point::new(x, y, a, b);
}

#[test]
fn point_new2() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x, y) = (FieldElement::new(17, prime), FieldElement::new(56, prime));

    let _p2 = Point::new(x, y, a, b);
}

#[test]
#[should_panic]
fn point_new3() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x, y) = (FieldElement::new(200, prime), FieldElement::new(119, prime));

    let _p3 = Point::new(x, y, a, b);
}

#[test]
fn point_new4() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x, y) = (FieldElement::new(1, prime), FieldElement::new(193, prime));

    let _p4 = Point::new(x, y, a, b);
}

#[test]
#[should_panic]
fn point_new5() {
    let prime = 223;
    let (a, b) = (FieldElement::new(0, prime), FieldElement::new(7, prime));
    let (x, y) = (FieldElement::new(42, prime), FieldElement::new(99, prime));

    let _p5 = Point::new(x, y, a, b);
}

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
