use std::{default, fmt::Display, ops::{Add, Mul, Sub}};

#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub enum VectorKind{
    Row,
    #[default]
    Column
}

impl Display for VectorKind{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self{
            VectorKind::Row => "Row",
            VectorKind::Column => "Column"
        };
        write!(f, "{}", s)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Vector{
    contents: Vec<f64>,
    kind: VectorKind,
}

impl Display for Vector{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String = "".into();
        for i in 0..self.contents.len(){
            let substr = format!("{}; ", self.contents[i]);
            s.push_str(substr.as_str())
        }
        s.pop();
        s.pop();
        write!(f, "{} ({})", self.kind, s)
    }
}

impl Vector{
    pub fn new(contents: Vec<f64>, kind: VectorKind) -> Self{
        Vector{contents, kind}
    }
    pub fn magnitude(&self) -> f64{
        let mut mag = 0.0;
        for i in &self.contents{
            mag += i.powi(2);
        }
        mag = mag.sqrt();
        mag
    }
    pub fn normalize(&mut self){
        let mag = self.magnitude();
        for i in &mut self.contents{
            *i /= mag;
        }
    }
    pub fn scale(&mut self, scalar: f64){
        for i in &mut self.contents{
            *i *= scalar;
        }
    }
    pub fn angle(lhs: &Vector, rhs: &Vector) -> f64{
        let mag = rhs.magnitude() * lhs.magnitude();
        let dot = rhs.clone() * lhs.clone();
        let cos: f64 = dot / mag;
        cos.acos()
    }
    pub fn transpone(&mut self){
        self.kind = match self.kind{
            VectorKind::Row => VectorKind::Column,
            VectorKind::Column => VectorKind::Row
        }
    }
}

impl Add for Vector{
    type Output = Vector;
    fn add(self, other: Self) -> Self::Output{
        if self.contents.len() != other.contents.len(){
            unimplemented!()
        }
        if self.kind != other.kind{
            unimplemented!()
        }
        let mut contents = Vec::new();
        for i in 0..self.contents.len(){
            contents.push(self.contents[i] + other.contents[i]);
        }
        Vector{contents, kind: self.kind}
    }
}

impl Sub for Vector{
    type Output = Vector;
    fn sub(self, other: Self) -> Self::Output{
        if self.contents.len() != other.contents.len(){
            unimplemented!()
        }
        if self.kind != other.kind{
            unimplemented!()
        }
        let mut contents = Vec::new();
        for i in 0..self.contents.len(){
            contents.push(self.contents[i] - other.contents[i]);
        }
        Vector{contents, kind: self.kind}
    }
}

// Dot product
impl Mul for Vector{
    type Output = f64;
    fn mul(self, other: Vector) -> Self::Output{
        let mut sum = 0.0;
        for i in 0..self.contents.len(){
            sum += self.contents[i] * other.contents[i];
        }
        sum
    }
}
