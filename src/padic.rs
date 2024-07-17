use crate::frac::*;
use rayon::prelude::*;

#[derive(Debug)]
pub struct Padic {
    pub v: Frac,
    pub p: i64,
    pub expanded: Vec<i64>,
}

#[allow(dead_code)]
pub fn expand(f: &Frac, p: i64, precision: usize) -> Padic {
    let mut a = f.numer;
    let b = f.denom;
    let mut expanded = Vec::new();

    for _ in 0..precision {
        let a2 = (0..p)
            .into_par_iter()
            .map(|k| ((a - k * b) as f64) / p as f64)
            .collect::<Vec<f64>>();
        // for all k, a2 = (a/b - k)*(b/p)
        let a3 = (0..p)
            .into_par_iter()
            .find_first(|&k| a2[k as usize].fract() == 0.0)
            .unwrap() as i64;
        // for all k, a3 = (a2/b - k)*(b/p)
        expanded.push(a3);
        a = (a - a3 * b) / p;
    }

    Padic { v: *f, p, expanded }
}

#[allow(dead_code)]
pub fn print_p_adic(padic: &Padic) {
    let reversed: Vec<i64> = padic.expanded.iter().cloned().rev().collect();
    let str: Vec<String> = reversed.par_iter().map(|&k| k.to_string()).collect();
    println!("...{}", str.join(""));
}

#[allow(dead_code)]
fn converter(expanded: Vec<i64>) -> i64 {
    expanded.iter().rev().fold(0, |acc, &d| acc * 10 + d)
}

#[allow(dead_code)]
fn back_converter(n: i64) -> Vec<i64> {
    let num = n.to_string().chars().rev().collect::<String>();
    let num_chars: Vec<char> = num.chars().collect();
    let expanded: Vec<i64> = num_chars
        .par_iter()
        .filter_map(|c| c.to_digit(10).map(|d| d as i64))
        .collect();
    expanded
}

#[allow(dead_code)]
pub fn add_p_adic(p1: &Padic, p2: &Padic) -> Padic {
    let (i1, i2) = rayon::join(
        || converter(p1.expanded.clone()),
        || converter(p2.expanded.clone()),
    );

    let expanded: Vec<i64> = back_converter(i1 + i2);

    Padic {
        v: Frac::add(&p1.v, &p2.v),
        p: p1.p,
        expanded,
    }
}

#[allow(dead_code)]
pub fn print_as_frac(p: &Padic) {
    println!("{}", p.v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_frac() {
        let a = Frac { numer: 1, denom: 2 };
        let b = Frac { numer: 3, denom: 9 };
        assert_eq!(Frac::add(&a, &b), Frac { numer: 5, denom: 6 });
        assert_eq!(Frac::add(&a, &a), Frac { numer: 1, denom: 1 });
        assert_eq!(Frac::add(&b, &b), Frac { numer: 2, denom: 3 });
    }

    #[test]
    fn testing_expand_and_arithmetic() {
        let frac = Frac {
            numer: 128,
            denom: 9,
        };
        let p = 7;
        let padic = expand(&frac, p, 10);
        assert_eq!(padic.expanded, vec![1, 5, 5, 0, 3, 5, 0, 3, 5, 0]);
        assert_eq!(
            padic.v,
            Frac {
                numer: 128,
                denom: 9
            }
        );

        let total = add_p_adic(&padic, &padic);
        assert_eq!(total.expanded, vec![2, 0, 1, 1, 6, 0, 1, 6, 0, 1]);
        assert_eq!(
            total.v,
            Frac {
                numer: 256,
                denom: 9
            }
        );

        let third = Frac { numer: 1, denom: 3 };
        let ten_adic = expand(&third, 10, 5);
        assert_eq!(ten_adic.expanded, vec![7, 6, 6, 6, 6]);

        let neg = Frac {
            numer: -1,
            denom: 1,
        };
        assert_eq!(expand(&neg, 10, 72).expanded, vec![9; 72]);
    }
}
