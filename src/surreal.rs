use core::cmp::Ordering;
use core::panic;
use rayon::prelude::*;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Surreal {
    pub l: Option<Vec<SurrealValue>>,
    pub r: Option<Vec<SurrealValue>>,
}

#[derive(Debug, Clone)]
pub enum SurrealValue {
    Float(f64),
    Surreal(Surreal),
}

impl Eq for SurrealValue {}

impl PartialEq for SurrealValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SurrealValue::Float(f1), SurrealValue::Float(f2)) => f1 == f2,
            (SurrealValue::Surreal(s1), SurrealValue::Surreal(s2)) => s1 == s2,
            _ => false,
        }
    }
}

impl PartialOrd for SurrealValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (SurrealValue::Float(f1), SurrealValue::Float(f2)) => f1.partial_cmp(f2),
            (SurrealValue::Float(_), SurrealValue::Surreal(_)) => Some(Ordering::Less),
            (SurrealValue::Surreal(_), SurrealValue::Float(_)) => Some(Ordering::Greater),
            (SurrealValue::Surreal(s1), SurrealValue::Surreal(s2)) => s1.partial_cmp(s2),
        }
    }
}

pub fn construct(num: &str) -> Surreal {
    fn parse_value(chars: &mut Chars) -> SurrealValue {
        let mut buffer = String::new();
        while let Some(c) = chars.as_str().chars().next() {
            match c {
                '{' => {
                    return SurrealValue::Surreal(parse_surreal(chars));
                }
                '}' | '|' | ',' => break,
                _ => buffer.push(c),
            }
            chars.next();
        }
        buffer
            .trim()
            .parse::<f64>()
            .map(SurrealValue::Float)
            .expect("Invalid float value")
    }

    fn parse_surreal(chars: &mut Chars) -> Surreal {
        chars.next();
        let mut l = Vec::new();
        let mut r = Vec::new();
        let mut left = true;

        while let Some(c) = chars.as_str().chars().next() {
            match c {
                '}' => {
                    chars.next();
                    break;
                }
                '|' => {
                    left = false;
                    chars.next();
                }
                ',' => {
                    chars.next();
                }
                ' ' => {
                    chars.next();
                }
                _ => {
                    if left {
                        l.push(parse_value(chars));
                    } else {
                        r.push(parse_value(chars));
                    }
                }
            }
        }
        Surreal {
            l: if l.is_empty() { None } else { Some(l) },
            r: if r.is_empty() { None } else { Some(r) },
        }
    }

    let mut chars = num.chars();
    parse_surreal(&mut chars)
}

fn value_to_string(v: &SurrealValue) -> String {
    match v {
        SurrealValue::Float(i) => i.to_string(),
        SurrealValue::Surreal(s) => format!("{}", s),
    }
}

pub fn negate(n: &Surreal) -> Surreal {
    let Surreal { l, r } = n;
    let negated = |val: &SurrealValue| -> SurrealValue {
        match val {
            SurrealValue::Float(i) => SurrealValue::Float(-i),
            SurrealValue::Surreal(s) => SurrealValue::Surreal(negate(s)),
        }
    };
    let (nlhs, nrhs) = rayon::join(
        || l.as_ref().map(|v| v.par_iter().map(negated).collect()),
        || r.as_ref().map(|v| v.par_iter().map(negated).collect()),
    );
    Surreal { l: nrhs, r: nlhs }
}

impl std::fmt::Display for Surreal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Surreal { l, r } = self;
        let left: String = l
            .as_ref()
            .map(|v| {
                v.par_iter()
                    .map(value_to_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .unwrap_or_else(String::new);
        let right: String = r
            .as_ref()
            .map(|v| {
                v.par_iter()
                    .map(value_to_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .unwrap_or_else(String::new);
        write!(f, "{{ {} | {} }}", left, right)
    }
}

pub fn print(n: &Surreal) {
    println!("{}", n);
}

pub fn append(surreal: &mut Surreal, value: SurrealValue, to_left: bool) {
    if to_left {
        if let Some(l) = &mut surreal.l {
            l.push(value);
        } else {
            surreal.l = Some(vec![value]);
        }
    } else if let Some(r) = &mut surreal.r {
        r.push(value);
    } else {
        surreal.r = Some(vec![value]);
    }
}

pub fn zero() -> Surreal {
    Surreal { l: None, r: None }
}

fn convert(n: &Surreal) -> f64 {
    if n.l.is_none() && n.r.is_none() {
        return 0.0;
    }
    if n.l.is_none() {
        match &n.r {
            Some(vec) if vec.len() == 1 => match &vec[0] {
                SurrealValue::Float(v) => *v - 1.0,
                _ => panic!("Too complicated"),
            },
            _ => panic!("Too complicated"),
        }
    } else if n.r.is_none() {
        match &n.l {
            Some(vec) if vec.len() == 1 => match &vec[0] {
                SurrealValue::Float(v) => *v + 1.0,
                _ => panic!("Too complicated"),
            },
            _ => panic!("Too complicated"),
        }
    } else {
        panic!("Too complicated");
    }
}

fn increment(side: &Option<Vec<SurrealValue>>, x: f64) -> Vec<SurrealValue> {
    side.as_ref().map_or_else(Vec::new, |values| {
        values
            .par_iter()
            .map(|v| match v {
                SurrealValue::Float(f) => SurrealValue::Float(f + x),
                SurrealValue::Surreal(_) => panic!("undefined for nested surreals"),
            })
            .collect()
    })
}

pub fn add(n1: &Surreal, n2: &Surreal) -> Surreal {
    // definition: x + y = {Xl + y, x + Yl | Xr + y, x + Yr}
    let x = convert(n1);
    let y = convert(n2);
    pdt_add(n1, x, n2, y)
}

#[allow(dead_code)]
pub fn pdt_add(n1: &Surreal, x: f64, n2: &Surreal, y: f64) -> Surreal {
    let (left, right): (Vec<SurrealValue>, Vec<SurrealValue>) = rayon::join(
        || {
            increment(&n1.l, y)
                .into_iter()
                .chain(increment(&n2.l, x))
                .collect()
        },
        || {
            increment(&n1.r, x)
                .into_iter()
                .chain(increment(&n2.r, x))
                .collect()
        },
    );

    let mut leftr = Some(left.clone());
    let mut rightr = Some(right.clone());

    if left.is_empty() {
        leftr = None;
    }

    if right.is_empty() {
        rightr = None;
    }

    Surreal {
        l: leftr,
        r: rightr,
    }
}

#[allow(dead_code)]
pub fn eq(n1: &Surreal, n2: &Surreal) -> bool {
    n1.l.par_iter().all(|x| n2.l.par_iter().any(|y| x == y))
        && n1.r.par_iter().all(|x| n2.r.par_iter().any(|y| x == y))
}

#[allow(dead_code)]
pub fn le(n1: &Surreal, n2: &Surreal) -> bool {
    let x = convert(n1);
    let y = convert(n2);

    let (check_left, check_right): (bool, bool) = rayon::join(
        || {
            n1.l.as_ref().map_or(true, |l_vals| {
                l_vals.par_iter().all(|v| v <= &SurrealValue::Float(y))
            })
        },
        || {
            n2.r.as_ref().map_or(true, |r_vals| {
                r_vals.par_iter().all(|v| &SurrealValue::Float(x) <= v)
            })
        },
    );

    check_left && check_right
}

#[allow(dead_code)]
pub fn ge(n1: &Surreal, n2: &Surreal) -> bool {
    le(n2, n1)
}

#[allow(dead_code)]
pub fn lt(n1: &Surreal, n2: &Surreal) -> bool {
    let x = convert(n1);
    let y = convert(n2);

    let (check_left, check_right): (bool, bool) = rayon::join(
        || {
            n1.l.as_ref().map_or(true, |l_vals| {
                l_vals.par_iter().all(|v| v < &SurrealValue::Float(y))
            })
        },
        || {
            n2.r.as_ref().map_or(true, |r_vals| {
                r_vals.par_iter().all(|v| &SurrealValue::Float(x) < v)
            })
        },
    );

    check_left && check_right
}

#[allow(dead_code)]
pub fn gt(n1: &Surreal, n2: &Surreal) -> bool {
    lt(n2, n1)
}

#[allow(dead_code)]
pub fn compare<F>(n1: &Surreal, n2: &Surreal, comparator: F) -> Surreal
where
    F: Fn(&Surreal, &Surreal) -> bool + Sync,
{
    let result: bool = comparator(n1, n2);
    if result {
        n1.clone()
    } else {
        n2.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_construct() {
        let result = construct("{ | }");
        assert_eq!(result, zero());
        let result2 = construct("{ 0.0 | { 1 | } }");
        assert_eq!(
            result2,
            Surreal {
                l: Some(vec![SurrealValue::Float(0.0)]),
                r: Some(vec![SurrealValue::Surreal(Surreal {
                    l: Some(vec![SurrealValue::Float(1.0)]),
                    r: None,
                })]),
            }
        );
        let result3 = construct("{ 2.0, 3.0, 4.0 | 9.0, 2.0 }");
        assert_eq!(
            result3,
            Surreal {
                l: Some(vec![
                    SurrealValue::Float(2.0),
                    SurrealValue::Float(3.0),
                    SurrealValue::Float(4.0),
                ]),
                r: Some(vec![SurrealValue::Float(9.0), SurrealValue::Float(2.0),]),
            }
        );
        let result4 = construct("{ | { | { | 6 } } }");
        assert_eq!(
            result4,
            Surreal {
                l: None,
                r: Some(vec![SurrealValue::Surreal(Surreal {
                    l: None,
                    r: Some(vec![SurrealValue::Surreal(Surreal {
                        l: None,
                        r: Some(vec![SurrealValue::Float(6.0),]),
                    })]),
                })]),
            }
        );
    }

    #[test]
    fn testing_negate() {
        let zero = zero();
        assert_eq!(zero, negate(&zero));
        let one = construct("{ 0.0 | }");
        assert_eq!(negate(&one), construct("{ | 0.0 }"));
        let nested = construct("{ 1.0, 2.0 | { 0.0 | } }");
        assert_eq!(negate(&nested), construct("{ { | 0.0 } | -1.0, -2.0 }"));
    }

    #[test]
    fn testing_append() {
        let mut result = construct("{ | }");
        append(&mut result, SurrealValue::Float(0.0), true);
        assert_eq!(result, construct("{ 0.0 | }"));
        append(
            &mut result,
            SurrealValue::Surreal(Surreal {
                l: None,
                r: Some(vec![SurrealValue::Float(9.0)]),
            }),
            false,
        );
        assert_eq!(result, construct("{ 0.0 | { | 9.0 } }"));
    }

    #[test]
    fn testing_conversion() {
        assert_eq!(convert(&construct("{ | }")), 0.0);
        assert_eq!(convert(&construct("{0.0 | }")), 1.0);
        assert_eq!(convert(&construct("{4.0 | }")), 5.0);
        assert_eq!(convert(&construct("{ | 0.0}")), -1.0);
        assert_eq!(convert(&construct("{ | -2.0}")), -3.0);
        assert_eq!(convert(&construct("{ | -20.0}")), -21.0);
    }

    #[test]
    fn testing_comparisons() {
        let n1 = construct("{0.0 | }");
        let n2 = construct("{0.0 | }");
        let n3 = construct("{ | 0.0 }");
        let n4 = construct("{1.0 | }");
        let n5 = construct("{ | -2.0 }");
        assert!(eq(&n1, &n1));
        assert!(eq(&n1, &n2));
        assert!(le(&n1, &n1));
        assert!(le(&n1, &n1));
        assert!(le(&n3, &n1));
        assert!(lt(&n3, &n1));
        assert!(lt(&n5, &n3));
        assert!(ge(&n2, &n1));
        assert!(ge(&n4, &n1));
        assert!(gt(&n3, &n5));
        assert!(gt(&n4, &n1));
    }

    #[test]
    fn testing_arithmetics() {
        let x = construct("{0.0 | }");
        let y = construct("{ 1.0 |  }");
        assert_eq!(add(&x, &y), construct("{2.0, 2.0 | }"));

        let z = construct("{ | }");
        assert_eq!(add(&z, &z), zero());
        assert_eq!(add(&z, &x), x);
        assert_eq!(add(&y, &z), y);
        assert_eq!(
            add(&construct("{1.0 | }"), &construct("{ | -2.0}")),
            construct("{ -2.0 | 0.0 }")
        );

        assert_eq!(
            pdt_add(
                &construct("{1.0, 2.0, 3.0, 4.0, 5.0 | }"),
                6.0,
                &construct("{ | -2.0}"),
                -3.0
            ),
            construct("{-2.0, -1.0, 0.0, 1.0, 2.0 | 4.0}")
        );

        let half = construct("{0|1}"); // 0.5
        assert_eq!(
            pdt_add(&half, 0.5, &half, 0.5),
            construct("{0.5, 0.5 | 1.5, 1.5}")
        );
    }
}
