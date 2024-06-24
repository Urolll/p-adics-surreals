mod surreal;
use crate::surreal::*;

fn main() {
    let x = construct("{ 1, 2 | }");
    let y = construct("{ 2, 3, 4 | 9, 2 }");
    let zero = construct("{ | }");
    let alt_zero = Surreal {
        l: Some(vec![]),
        r: Some(vec![]),
    };
    let k: Surreal = Surreal {
        l: Some(vec![
            SurrealValue::Integer(1),
            SurrealValue::Integer(2),
            SurrealValue::Integer(3),
        ]),
        r: Some(vec![]),
    };

    let neg_x = negate(&x);
    print(&x);
    print(&neg_x);

    let neg_y = negate(&y);
    print(&y);
    print(&neg_y);

    print(&alt_zero);
    print(&negate(&zero));

    print(&negate(&k));

    let nested = construct("{ 1 | { 0 | { -1 | } } }");
    print(&nested);
    print(&negate(&nested));
}
