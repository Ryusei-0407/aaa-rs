#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    x: isize,
    y: isize,
    a: isize,
    b: isize,
}

impl Point {
    fn new(x: isize, y: isize, a: isize, b: isize) -> Point {
        if x == 0 && y == 0 {
            Point { x, y, a, b }
        } else if y.pow(2) != x.pow(3) + a * x + b {
            panic!("({x}, {y}) is not on the curve")
        } else {
            Point { x, y, a, b }
        }
    }

    fn add(&self, other: &Point) -> Point {
        if self.a != other.a || self.b != other.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, other)
        };
        let a = self.a;
        let b = self.b;

        if self == other && self.y == 0 {
            return Point::new(0, 0, a, b);
        }

        if self.x == other.x && self.y != other.y {
            return Point::new(0, 0, a, b);
        }

        let x = (self.x, other.x);
        match x {
            (0, ..) => Point::new(other.x, other.y, a, b),
            (.., 0) => Point::new(self.x, self.y, a, b),
            _ => {
                let s = if self.y == other.y {
                    (3 * self.x.pow(2) + a) / (2 * self.y)
                } else {
                    (other.y - self.y) / (other.x - self.x)
                };
                let x = s.pow(2) - self.x - other.x;
                let y = s * (self.x - x) - self.y;

                Point::new(x, y, a, b)
            }
        }
    }
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

#[test]
fn point_add1() {
    let p1 = Point::new(2, 5, 5, 7);
    let p2 = Point::new(-1, -1, 5, 7);
    let ans = Point::new(3, -7, 5, 7);
    let ans2 = Point::new(18, 77, 5, 7);

    assert_eq!(Point::add(&p1, &p2), ans);
    assert_eq!(Point::add(&p2, &p2), ans2);
}

#[test]
fn point_add2() {
    let inf = Point::new(0, 0, 5, 7);

    assert_eq!(Point::add(&inf, &inf), inf);
}

#[test]
fn point_add_inf() {
    let p1 = Point::new(-1, -1, 5, 7);
    let p2 = Point::new(-1, 1, 5, 7);
    let inf = Point::new(0, 0, 5, 7);

    assert_eq!(Point::add(&p1, &inf), p1);
    assert_eq!(Point::add(&inf, &p2), p2);
    assert_eq!(Point::add(&p1, &p2), inf);
}
