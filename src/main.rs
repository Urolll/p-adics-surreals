mod pstar;
mod surreal;
use crate::pstar::*;
use crate::surreal::*;

fn main() {
    let x = construct("{ 1, 2 | }");
    let y = construct("{ 2, 3, 4 | 9, 2 }");
    let zero = zero();
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

    let star_1 = star(1);
    print(&star_1);
    let star_2 = star(2);
    print(&star_2);
    let star_3 = star(3);
    print(&star_3);
    let star = astar(2, 3);
    print(&negate(&star));

    let pstar = pretty_star(2);
    let _pastar = pretty_astar(2, 3);
    print_pstar(&pstar);
    print(&expand_pstar(pstar));
}
