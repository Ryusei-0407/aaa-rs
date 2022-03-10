#[derive(Debug, PartialEq)]
pub struct FieldElement {
    num: isize,
    prime: isize,
}

impl FieldElement {
    pub fn new(num: isize, prime: isize) -> FieldElement {
        if num >= prime || num < 0 {
            println!("Num {} not in field range 0 to {}", num, prime - 1)
        };

        FieldElement { num, prime }
    }
}

#[test]
fn ecc() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(6, 13);

    assert_eq!(a, a);
    assert_ne!(a, b);
}
