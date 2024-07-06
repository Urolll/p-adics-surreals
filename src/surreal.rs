use core::panic;
use rayon::prelude::*;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Surreal {
    pub l: Option<Vec<SurrealValue>>,
    pub r: Option<Vec<SurrealValue>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurrealValue {
    Integer(i32),
    Surreal(Surreal),
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
            .parse::<i32>()
            .map(SurrealValue::Integer)
            .expect("Invalid integer value")
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
        SurrealValue::Integer(i) => i.to_string(),
        SurrealValue::Surreal(s) => format!("{}", s),
    }
}

pub fn negate(n: &Surreal) -> Surreal {
    let Surreal { l, r } = n;
    let negated = |val: &SurrealValue| -> SurrealValue {
        match val {
            SurrealValue::Integer(i) => SurrealValue::Integer(-i),
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

fn convert(n: &Surreal) -> i32 {
    if n.l.is_none() && n.r.is_none() {
        return 0;
    }
    if n.l.is_none() {
        match &n.r {
            Some(vec) if vec.len() == 1 => match &vec[0] {
                SurrealValue::Integer(v) => *v - 1,
                _ => panic!("Too complicated expression to use arithmetic"),
            },
            _ => panic!("Too complicated expression to use arithmetic"),
        }
    } else if n.r.is_none() {
        match &n.l {
            Some(vec) if vec.len() == 1 => match &vec[0] {
                SurrealValue::Integer(v) => *v + 1,
                _ => panic!("Too complicated expression to use arithmetic"),
            },
            _ => panic!("Too complicated expression to use arithmetic"),
        }
    } else {
        panic!("Surreal number has both left and right parts; cannot convert to integer");
    }
}

fn increment(side: &Option<Vec<SurrealValue>>, x: i32) -> Vec<SurrealValue> {
    side.as_ref().map_or_else(Vec::new, |values| {
        values
            .par_iter()
            .map(|v| match v {
                SurrealValue::Integer(i) => SurrealValue::Integer(i + x),
                SurrealValue::Surreal(_) => panic!("undefined for nesed surreals"),
            })
            .collect()
    })
}

pub fn add(n1: &Surreal, n2: &Surreal) -> Surreal {
    // definition: x + y = {Xl + y, x + Yl | Xr + y, x + Yr}
    let x = convert(n1);
    let y = convert(n2);
    let (left, right): (Vec<SurrealValue>, Vec<SurrealValue>) = rayon::join(
        || {
            increment(&n1.l, y)
                .into_iter()
                .chain(increment(&n2.l, x))
                .collect()
        },
        || {
            increment(&n1.r, y)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_construct() {
        let result = construct("{ | }");
        assert_eq!(result, zero());
        let result2 = construct("{ 0 | { 1 | } }");
        assert_eq!(
            result2,
            Surreal {
                l: Some(vec![SurrealValue::Integer(0)]),
                r: Some(vec![SurrealValue::Surreal(Surreal {
                    l: Some(vec![SurrealValue::Integer(1)]),
                    r: None,
                })]),
            }
        );
        let result3 = construct("{ 2, 3, 4 | 9, 2 }");
        assert_eq!(
            result3,
            Surreal {
                l: Some(vec![
                    SurrealValue::Integer(2),
                    SurrealValue::Integer(3),
                    SurrealValue::Integer(4),
                ]),
                r: Some(vec![SurrealValue::Integer(9), SurrealValue::Integer(2),]),
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
                        r: Some(vec![SurrealValue::Integer(6),]),
                    })]),
                })]),
            }
        );
    }

    #[test]
    fn testing_negate() {
        let zero = zero();
        assert_eq!(zero, negate(&zero));
        let one = construct("{ 0 | }");
        assert_eq!(negate(&one), construct("{ | 0 }"));
        let nested = construct("{ 1, 2 | { 0 | } }");
        assert_eq!(negate(&nested), construct("{ { | 0 } | -1, -2 }"));
    }

    #[test]
    fn testing_append() {
        let mut result = construct("{ | }");
        append(&mut result, SurrealValue::Integer(0), true);
        assert_eq!(result, construct("{ 0 | }"));
        append(
            &mut result,
            SurrealValue::Surreal(Surreal {
                l: None,
                r: Some(vec![SurrealValue::Integer(9)]),
            }),
            false,
        );
        assert_eq!(result, construct("{ 0 | { | 9 } }"));
    }

    #[test]
    fn testing_conversion() {
        assert_eq!(convert(&construct("{ | }")), 0);
        assert_eq!(convert(&construct("{0 | }")), 1);
        assert_eq!(convert(&construct("{4 | }")), 5);
        assert_eq!(convert(&construct("{ | 0}")), -1);
        assert_eq!(convert(&construct("{ | -2}")), -3);
        assert_eq!(convert(&construct("{ | -20}")), -21);
    }

    #[test]
    fn testing_arithmetics() {
        let x = construct("{0 | }");
        let y = construct("{ 1 |  }");
        assert_eq!(add(&x, &y), construct("{2, 2 | }"));

        let z = construct("{ | }");
        assert_eq!(add(&z, &z), zero());
        assert_eq!(add(&z, &x), x);
        assert_eq!(add(&y, &z), y);
        assert_eq!(
            add(&construct("{1 | }"), &construct("{ | -2}")),
            construct("{ -2 | 0 }")
        );
    }
}
