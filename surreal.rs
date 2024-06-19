#[derive(Debug)]
struct Surreal {
    l: Option<Vec<i32>>,
    r: Option<Vec<i32>>,
}

fn construct(num: &str) -> Surreal {
    let trimmed = num
        .trim_matches(|c| c == '{' || c == '}' || c == ' ')
        .trim();
    let parts: Vec<&str> = trimmed.split('|').collect();
    let result = |part: &str| -> Vec<i32> {
        part.split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect::<Vec<i32>>()
    };
    let lhs = if parts.len() > 0 && !parts[0].trim().is_empty() {
        Some(result(parts[0]))
    } else {
        None
    };
    let rhs = if parts.len() > 1 && !parts[1].trim().is_empty() {
        Some(result(parts[1]))
    } else {
        None
    };
    construct_from_vec(lhs, rhs)
}

fn construct_from_vec(left: Option<Vec<i32>>, right: Option<Vec<i32>>) -> Surreal {
    Surreal { l: left, r: right }
}

fn vec_to_string(v: Vec<i32>) -> String {
    v.iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

fn negate(n: &Surreal) -> Surreal {
    let Surreal { l, r } = n;
    let nlhs = match l {
        Some(l) => Some(l.iter().map(|&num| -num).collect::<Vec<i32>>()),
        None => None,
    };
    let nrhs = match r {
        Some(r) => Some(r.iter().map(|&num| -num).collect::<Vec<i32>>()),
        None => None,
    };
    Surreal { l: nrhs, r: nlhs }
}

fn print(n: &Surreal) {
    let Surreal { l, r } = n;
    let left: String = match l {
        Some(l) => vec_to_string(l.to_vec()),
        None => String::new(),
    };
    let right: String = match r {
        Some(r) => vec_to_string(r.to_vec()),
        None => String::new(),
    };
    println!("{{{} | {}}}", left, right);
}

fn test() {
    let x = construct("{ 1, 2 | }");
    let y = construct("{ 2, 3, 4 | 9, 2 }");
    let zero = construct("{ | }");

    let neg_x = negate(&x);
    print(&x);
    print(&neg_x);

    let neg_y = negate(&y);
    print(&y);
    print(&neg_y);

    print(&zero);
    print(&negate(&zero));
}

fn main() {
    test();
}
