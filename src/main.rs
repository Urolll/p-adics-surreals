mod frac;
mod padic;
mod pstar;
mod surreal;
use crate::frac::*;
use crate::padic::*;
use crate::pstar::*;
use crate::surreal::*;

extern crate surreal as crate_surreal;
use crate_surreal::Surreal as CrateSurreal;

extern crate padic as crate_padic;
use crate_padic::Ratio;
use std::time::Instant;

#[allow(dead_code)]
fn create_complex_par_surreal() -> Surreal {
    let mut nested_set: Vec<Surreal> = vec![];

    for i in 0..5 {
        let mut left_set: Vec<Surreal> = vec![];
        let mut right_set: Vec<SurrealValue> = vec![];

        for j in 0..5 {
            let left_value = SurrealValue::Float(i as f64);
            let right_value = SurrealValue::Float((i * 5 + j) as f64);

            let left_surreal = Surreal {
                l: Some(vec![left_value.clone()]),
                r: Some(
                    nested_set
                        .iter()
                        .map(|s| SurrealValue::Surreal(s.clone()))
                        .collect(),
                ),
            };

            left_set.push(left_surreal);

            right_set.push(right_value);
        }

        let nested_surreal = Surreal {
            l: Some(vec![SurrealValue::Float(i as f64)]),
            r: Some(right_set),
        };

        nested_set.push(nested_surreal);
    }

    Surreal {
        l: Some(vec![SurrealValue::Float(0.0)]),
        r: Some(
            nested_set
                .into_iter()
                .map(|s| SurrealValue::Surreal(s))
                .collect(),
        ),
    }
}

#[allow(dead_code)]
fn create_complex_crate_surreal() -> CrateSurreal {
    let mut nested_set: Vec<CrateSurreal> = vec![];

    for _i in 0..5 {
        let mut left_set: Vec<CrateSurreal> = vec![];
        let mut right_set: Vec<CrateSurreal> = vec![];

        for _j in 0..5 {
            let left_surreal = CrateSurreal::new(vec![], vec![]);
            left_set.push(left_surreal);

            let right_surreal_1 = CrateSurreal::new(vec![], vec![]);
            let right_surreal_2 = CrateSurreal::new(vec![], vec![]);
            let right_surreal_3 = CrateSurreal::new(vec![], vec![]);
            let right_surreal = CrateSurreal::new(
                vec![&right_surreal_1, &right_surreal_2],
                vec![&right_surreal_3],
            );
            right_set.push(right_surreal);
        }

        let nested_surreal_1 = CrateSurreal::new(vec![], vec![]);
        let nested_surreal_2 = CrateSurreal::new(vec![], vec![]);
        let nested_surreal_3 = CrateSurreal::new(vec![], vec![]);
        let nested_surreal = CrateSurreal::new(
            vec![&nested_surreal_1],
            vec![&nested_surreal_2, &nested_surreal_3],
        );
        nested_set.push(nested_surreal);
    }

    let final_surreal_1 = CrateSurreal::new(vec![], vec![]);
    let final_surreal_2 = CrateSurreal::new(vec![], vec![]);
    let final_surreal_3 = CrateSurreal::new(vec![], vec![]);
    CrateSurreal::new(
        vec![&final_surreal_1],
        vec![&final_surreal_2, &final_surreal_3],
    )
}

fn main() {
    let start = Instant::now();
    let zero = zero();
    let one = Surreal {
        l: Some(vec![SurrealValue::Float(0.0)]),
        r: None,
    };
    let _ = lt(&zero, &one);
    let _ = gt(&zero, &one);
    let _ = le(&zero, &one);
    let _ = ge(&zero, &one);
    let elapsed = start.elapsed();
    println!(
        "Time taken for par surreal to perform comparisons: {:.6?}",
        elapsed
    );

    let start = Instant::now();
    let zero = CrateSurreal::new(vec![], vec![]);
    let one = CrateSurreal::new(vec![&zero], vec![]);
    let _ = CrateSurreal::lt(&zero, &one);
    let _ = CrateSurreal::gt(&zero, &one);
    let _ = CrateSurreal::le(&zero, &one);
    let _ = CrateSurreal::ge(&zero, &one);
    let elapsed = start.elapsed();
    println!(
        "Time taken for crate surreal to perform comparisons: {:.6?}",
        elapsed
    );

    let start = Instant::now();

    let n1 = create_complex_par_surreal();
    let n2 = create_complex_par_surreal();

    //print(&n1);
    //print(&n2);
    let _ = negate(&n1);
    let _ = negate(&n2);

    let elapsed = start.elapsed();
    println!(
        "Time taken for par surreal to perform nested negation: {:.6?}",
        elapsed
    );

    let start = Instant::now();
    let _ = star(1);
    let _ = star(2);
    let _ = star(3);
    let _ = astar(2, 3);

    let _ = pretty_star(2);
    let _ = pretty_astar(2, 3);

    let elapsed = start.elapsed();
    println!(
        "Time taken for par surreal to perform star operations: {:.6?}",
        elapsed
    );

    let start = Instant::now();
    let frac = Frac { numer: 2, denom: 5 };
    let _ = expand(&frac, 3, 12);
    let elapsed = start.elapsed();
    println!(
        "Time taken for par padic to expand to 12 precision: {:.6?}",
        elapsed
    );

    let start = Instant::now();
    let frac = Ratio::new(2, 5);
    let _ = frac.to_padic(3, 12).expansion;
    let elapsed = start.elapsed();
    println!(
        "Time taken for crate padic to expand to 12 precision: {:.6?}",
        elapsed
    );

    let start = Instant::now();
    let frac = Frac { numer: 2, denom: 5 };
    let _ = expand(&frac, 7, 20000);
    let elapsed = start.elapsed();
    println!(
        "Time taken for par padic to expand to 20000 precision: {:.6?}",
        elapsed
    );

    let start = Instant::now();
    let frac = Ratio::new(2, 5);
    let _ = frac.to_padic(7, 20000).expansion;
    let elapsed = start.elapsed();
    println!(
        "Time taken for crate padic to expand to 20000 precision: {:.6?}",
        elapsed
    );

    let start = Instant::now();
    let frac = Frac { numer: 2, denom: 5 };
    let a = expand(&frac, 7, 10);
    let _ = add_p_adic(&a, &a);
    let elapsed = start.elapsed();
    println!(
        "Time taken for par padic to perform addition with 10 precision: {:.6?}",
        elapsed
    );
}
