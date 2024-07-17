use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Frac {
    pub numer: i64,
    pub denom: i64,
}

impl Frac {
    pub fn add(&self, other: &Frac) -> Frac {
        let (nn, nd) = rayon::join(
            || self.numer * other.denom + other.numer * self.denom,
            || self.denom * other.denom,
        );
        Frac::reduce(nn, nd)
    }

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a.abs()
    }

    fn reduce(numer: i64, denom: i64) -> Frac {
        let g = Frac::gcd(numer, denom);
        Frac {
            numer: numer / g,
            denom: denom / g,
        }
    }
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numer, self.denom)
    }
}

impl PartialEq for Frac {
    fn eq(&self, other: &Self) -> bool {
        let (rs, ro) = rayon::join(
            || Frac::reduce(self.numer, self.denom),
            || Frac::reduce(other.numer, other.denom),
        );
        rs.numer == ro.numer && rs.denom == ro.denom
    }
}

impl Eq for Frac {}
