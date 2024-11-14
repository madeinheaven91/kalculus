use std::{fmt::Display, ops::{Add, Mul, Sub}};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Complex{
    pub re: f64,
    pub im: f64
}

impl Display for Complex{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i_str = match self.im.abs(){
            1.0 => "".into(),
            _ => format!("{}", self.im.abs())
        };
        if self.im == 0.0 && self.re == 0.0{
            write!(f, "0")?;
            return Ok(())
        }
        if self.im == 0.0{
            write!(f, "{}", self.re)?;
            return Ok(())
        }
        if self.re == 0.0{
            write!(f, "{}i", i_str)?;
            return Ok(())
        }
        let operator = match self.im > 0.0{
            true => "+",
            false => "-"
        };
        write!(f, "{}{}{}i", self.re, operator, i_str)
    }
}

impl Complex{
    pub fn conjugate(&self) -> Complex{
        Complex{
            re: self.re,
            im: -self.im
        }
    }
}

impl Add for Complex{
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Complex{
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl Sub for Complex{
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex{
            re: self.re - rhs.re,
            im: self.im - rhs.im
        }
    }
}

impl Mul for Complex{
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex{
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re
        }
    }
}
