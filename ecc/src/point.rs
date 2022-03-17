mod fieldelement;

use std::rc::Rc;

use fieldelement::*;

#[derive(Debug, PartialEq)]
pub struct Point {
    x: FieldElement,
    y: FieldElement,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    fn new(x: FieldElement, y: FieldElement, a: FieldElement, b: FieldElement) -> Point {
        if x.num == 0 && y.num == 0 {
            Point { x, y, a, b }
        } else if FieldElement::pow(&y, 2)
            != FieldElement::add(
                &FieldElement::add(&FieldElement::pow(&x, 3), &FieldElement::mul(&a, &x)),
                &b,
            )
        {
            panic!("({:?}, {:?}) is not on the curve", x, y)
        } else {
            Point { x, y, a, b }
        }
    }

    fn add(&self, other: &Point) -> Point {
        if self.a.num != other.a.num || self.b.num != other.b.num {
            panic!("Points {:?}, {:?} are not on the same curve", self, other)
        };
        let a = &self.a;
        let b = &self.b;
        let prime = &a.prime;
        let zero = Rc::new(FieldElement::new(0, *prime));

        if self == other && self.y.num == 0 {
            let z = Rc::clone(&zero);
            return Point::new(*z, *z, *a, *b);
        };

        if self.x == other.x && self.y != other.y {
            let z = Rc::clone(&zero);
            return Point::new(*z, *z, *a, *b);
        };

        let x = (self.x.num, other.x.num);
        match x {
            (0, ..) => {
                let (x, y) = (&other.x, &other.y);
                Point::new(*x, *y, *a, *b)
            }
            _ => Point::new(self.x, self.y, *a, *b),
        }
    }
}
