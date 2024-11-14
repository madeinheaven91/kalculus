use std::{f64, fmt::Display, ops::{Add, Div, Mul, Neg, Sub}};

// TODO:
// add support for trigonometric and exponential forms
// alg: z = a + bi
// trig: z = r(cos(a) + i sin(a))
// exp: z = re^(i * phi)
// idk how it should work and how it should be done, but it is needed
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Complex{
    pub re: f64,
    pub im: f64,
    pub form: ComplexForm
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum ComplexForm{
    #[default]
    Alg,
    Trig,
    Exp
}

//TODO:
//parse complex from string
impl Complex{
    pub fn new(re: f64, im: f64, form: ComplexForm) -> Self{
        Complex{re, im, form}
    }
    pub fn new_alg(re: f64, im: f64) -> Self{
        Complex{re, im, form: ComplexForm::Alg}
    }
    pub fn conjugate(&self) -> Complex{
        Complex{
            re: self.re,
            im: -self.im,
            form: self.form
        }
    }
    pub fn abs(&self) -> f64{
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }
    pub fn arg(&self) -> f64{
        (self.re / self.abs()).acos()
    }

    pub fn i() -> Self{
        Complex{re: 0.0, im: 1.0, form: ComplexForm::Alg}
    }
}

// TODO:
// C - complex number
// R - real number (f64)
// Z - integer (i64)
// ...
// should be able to add: 
//  C + C (+)
//  C + R 
//  R + C 
//  C + Z
//  Z + C
//  same for subtraction, multiplication and division
impl Add for Complex{
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        if self.form != rhs.form {
            panic!("Cannot add complex numbers with different forms")
        }
        match self.form{
            ComplexForm::Alg =>
                Complex{
                    re: self.re + rhs.re,
                    im: self.im + rhs.im,
                    form: self.form
                },
            _ => unimplemented!()
        }
    }
}

impl Sub for Complex{
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.form != rhs.form {
            panic!("Cannot subtract complex numbers with different forms")
        }
        match self.form{
            ComplexForm::Alg =>
                Complex{
                    re: self.re - rhs.re,
                    im: self.im - rhs.im,
                    form: self.form
                },
            _ => unimplemented!()
        }
    }
}

impl Mul for Complex{
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.form != rhs.form {
            panic!("Cannot multiply complex numbers with different forms")
        }
        match self.form{
            ComplexForm::Alg => Complex{
                re: self.re * rhs.re - self.im * rhs.im,
                im: self.re * rhs.im + self.im * rhs.re,
                form: self.form
                },
            _ => unimplemented!()
        }
    }
}

impl Div for Complex{
    type Output = Complex;
    fn div(self, rhs: Self) -> Self::Output {
        if self.form != rhs.form {
            panic!("Cannot divide complex numbers with different forms")
        }
        match self.form{
            ComplexForm::Alg => Complex{
                re: (self.re * rhs.re + self.im * rhs.im) / (rhs.re.powi(2) + rhs.im.powi(2)),
                im: (self.im * rhs.re - self.re * rhs.im) / (rhs.re.powi(2) + rhs.im.powi(2)),
                form: self.form
                },
            _ => unimplemented!()
        }
    }
}

impl Neg for Complex{
    type Output = Complex;
    fn neg(self) -> Self::Output {
        Complex{
            re: -self.re,
            im: -self.im,
            form: self.form
        }
    }
}

impl Display for Complex{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.form{
            ComplexForm::Alg => {
                if self.re == 0.0 && self.im == 0.0{
                    write!(f, "0")?;
                    return Ok(())
                };
                let re: String = match self.re{
                    0.0 => "".into(),
                    _ => self.re.to_string()
                };
                let im: String = match self.im{
                    0.0 => "".into(),
                    1.0 => match re.as_str(){
                        "" => "".into(),
                        _ => "+".into()
                    },
                    -1.0 => "-".into(),
                    _ => self.im.to_string()
                };
                let im_symbol: String = match self.im{
                    0.0 => "".into(),
                    _ => "i".into()
                };
                write!(f, "{}{}{}", re, im, im_symbol)
            }
            _ => unimplemented!()
        }
    }
}

impl From<f64> for Complex{
    fn from(arg: f64) -> Self {
        Complex{re: arg, im: 0.0, form: ComplexForm::Alg}
    }
}

impl From<i64> for Complex{
    fn from(arg: i64) -> Self {
        Complex{re: arg as f64, im: 0.0, form: ComplexForm::Alg}
    }
}
