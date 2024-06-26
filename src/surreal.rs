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
}
