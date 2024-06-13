use std::ops::{Mul, MulAssign};

#[derive(Clone, Eq)]
pub struct Fraction {
    numerator: u32,
    denominator: u32
}

impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Fraction {
        let gcd = Fraction::gcd(numerator, denominator);
        let numerator = numerator / gcd;
        let denominator = denominator / gcd;
        Fraction{numerator, denominator}
    }

    pub fn numerator(&self) -> u32 {
        self.numerator
    }

    pub fn denominator(&self) -> u32 {
        self.denominator
    }

    pub fn as_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    fn gcd(a: u32, b: u32) -> u32 {
        let mut a = a;
        let mut b = b;
        while b > 0 {
            let tmp = a;
            a = b;
            b = tmp % b;
        }
        a
    }
}

// operator*
impl Mul for Fraction {
    type Output = Self;

    //fn mul(self, other: Fraction) -> <Self as Mul<fraction::Fraction>>::Output {
    fn mul(self, other: Fraction) -> Self {
        let numerator = self.numerator * other.numerator;
        let denominator = self.denominator * other.denominator;
        Fraction::new(numerator, denominator)
    }
}

// operator*=
impl MulAssign for Fraction {
    fn mul_assign(&mut self, other: Fraction) {
        self.numerator *= other.numerator;
        self.denominator *= other.denominator;
    }
}

// operator ==
impl PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        self.numerator == other.numerator &&
        self.denominator == other.denominator
    }
}

impl From<u32> for Fraction {
    fn from(n: u32) -> Self {
        Self::new(n, 1)
    }
}

impl From<(u32,u32)> for Fraction {
    fn from(value: (u32,u32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl TryFrom<f32> for Fraction {
    type Error = String;

    fn try_from(a: f32) -> Result<Self, String> {
        let s = a.to_string();
        println!("{}", s);
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 2 {
            return Err("NaN".to_string());
        }

        let p = parts[1].len() as u32;
        let d0: u32 = parts[0].to_string().parse::<u32>().unwrap();
        let d1: u32 = parts[1].to_string().parse::<u32>().unwrap();

        let f1: Fraction = p.into();
        let f2: Fraction = Fraction::from((d1, u32::pow(10, p)));
        
        Ok(f1 + f2)

    }
}

impl TryFrom<&str> for Fraction {
    type Error = String;

    fn try_from(a: &str) -> Result<Self, String> {
        let parts: Vec<&str> = a.split('/').collect();
        let numerator = parts[0].parse::<u32>().unwrap();
        let denominator = parts[1].parse::<u32>().unwrap();

        Ok(Fraction::new(numerator, denominator))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let t = Fraction::new(1, 2);
        assert_eq!(t.numerator, 1);
        assert_eq!(t.denominator, 2);
        assert_eq!(t.as_f64(), 0.5);
    }

    #[test]
    fn gcd() {
        assert_eq!(Fraction::gcd(81, 18), 9);
        assert_eq!(Fraction::gcd(18, 27), 9);
    }

    #[test]
    fn partial_eq() {
        let mut a = Fraction::new(1, 2);
        let b = Fraction::new(2, 4);
        let one = Fraction{numerator:3, denominator:3};
        assert!(a == b);
        a *= one;
        // assert!(a == b);
    }
}