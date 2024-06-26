use crate::surreal::*;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PStar {
    pub l: Vec<StarValue>,
    pub r: Vec<StarValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StarValue {
    String(String),
    Integer(i32),
}

pub fn star(n: i32) -> Surreal {
    fn star_tail(n: i32, acc: Surreal) -> Surreal {
        match n {
            1 => acc,
            _ => star_tail(n - 1, {
                let mut new_acc = acc.clone();
                append(&mut new_acc, SurrealValue::Surreal(acc.clone()), true);
                append(&mut new_acc, SurrealValue::Surreal(acc.clone()), false);
                new_acc
            }),
        }
    }
    let star = Surreal {
        l: Some(vec![SurrealValue::Integer(0)]),
        r: Some(vec![SurrealValue::Integer(0)]),
    };
    if n == 1 {
        star
    } else {
        star_tail(n, star)
    }
}

pub fn astar(n1: i32, n2: i32) -> Surreal {
    star(n1 ^ n2)
}

pub fn pretty_star(n: i32) -> PStar {
    fn pretty_star_tail(n: i32, acc: PStar, current: i32) -> PStar {
        match n {
            1 => acc,
            _ => pretty_star_tail(
                n - 1,
                {
                    let mut new_acc = acc.clone();
                    new_acc
                        .l
                        .push(StarValue::String("*".to_owned() + &(current).to_string()));
                    new_acc
                        .r
                        .push(StarValue::String("*".to_owned() + &(current).to_string()));
                    new_acc
                },
                current + 1,
            ),
        }
    }

    pretty_star_tail(
        n,
        PStar {
            l: vec![StarValue::Integer(0)], //StarValue::String("*".to_owned())],
            r: vec![StarValue::Integer(0)], //StarValue::String("*".to_owned())],
        },
        1,
    )
}

pub fn pretty_astar(n1: i32, n2: i32) -> PStar {
    pretty_star(n1 ^ n2)
}

pub fn print_pstar(star: &PStar) {
    let l_string: String = star
        .l
        .par_iter()
        .map(|v| match v {
            StarValue::String(s) => s.clone(),
            StarValue::Integer(i) => i.to_string(),
        })
        .collect::<Vec<String>>()
        .join(", ");
    let r_string: String = star
        .r
        .par_iter()
        .map(|v| match v {
            StarValue::String(s) => s.clone(),
            StarValue::Integer(i) => i.to_string(),
        })
        .collect::<Vec<String>>()
        .join(", ");
    println!("{{ {} | {} }}", l_string, r_string);
}

fn star_to_surreal(value: &StarValue) -> SurrealValue {
    match value {
        StarValue::String(s) => {
            if s == "*" {
                SurrealValue::Surreal(star(1))
            } else {
                let n = s.trim_start_matches('*').parse::<i32>().unwrap();
                SurrealValue::Surreal(star(n))
            }
        }
        StarValue::Integer(i) => SurrealValue::Integer(*i),
    }
}

fn map_to_surreal(values: &[StarValue]) -> Vec<SurrealValue> {
    values.par_iter().map(star_to_surreal).collect()
}

pub fn expand_pstar(star: PStar) -> Surreal {
    let (l, r): (Option<Vec<SurrealValue>>, Option<Vec<SurrealValue>>) = rayon::join(
        || {
            if star.l.is_empty() {
                None
            } else {
                Some(map_to_surreal(&star.l))
            }
        },
        || {
            if star.r.is_empty() {
                None
            } else {
                Some(map_to_surreal(&star.r))
            }
        },
    );
    Surreal { l, r }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_star_astar() {
        let star1 = star(1);
        assert_eq!(star1, construct("{ 0 | 0 }"));
        let star2 = star(2);
        assert_eq!(star2, construct("{0, {0 | 0} | 0, {0 | 0}}"));
        let astar1 = astar(2, 3);
        assert_eq!(star1, astar1);
    }

    #[test]
    fn testing_pretty_star() {
        let pstar1 = pretty_star(1);
        let pstar2 = pretty_star(2);
        let pstar3 = pretty_star(3);
        assert_eq!(
            pstar1,
            PStar {
                l: vec![StarValue::Integer(0)],
                r: vec![StarValue::Integer(0)],
            }
        );
        assert_eq!(
            pstar2,
            PStar {
                l: vec![StarValue::Integer(0), StarValue::String("*1".to_owned())],
                r: vec![StarValue::Integer(0), StarValue::String("*1".to_owned())],
            }
        );
        assert_eq!(
            pstar3,
            PStar {
                l: vec![
                    StarValue::Integer(0),
                    StarValue::String("*1".to_owned()),
                    StarValue::String("*2".to_owned())
                ],
                r: vec![
                    StarValue::Integer(0),
                    StarValue::String("*1".to_owned()),
                    StarValue::String("*2".to_owned())
                ],
            }
        );
    }
}
