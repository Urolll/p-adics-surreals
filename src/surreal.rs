use std::str::Chars;

#[derive(Debug)]
pub struct Surreal {
    pub l: Option<Vec<SurrealValue>>,
    pub r: Option<Vec<SurrealValue>>,
    // change this to be a recursive struct later
}

#[derive(Debug)]
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
        chars.next(); // Skip '{'
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
    let nlhs = l.as_ref().map(|v| v.iter().map(negated).collect());
    let nrhs = r.as_ref().map(|v| v.iter().map(negated).collect());
    Surreal { l: nrhs, r: nlhs }
}

impl std::fmt::Display for Surreal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Surreal { l, r } = self;
        let left: String = l
            .as_ref()
            .map(|v| {
                v.iter()
                    .map(value_to_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .unwrap_or_else(String::new);
        let right: String = r
            .as_ref()
            .map(|v| {
                v.iter()
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
