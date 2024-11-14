pub mod vector;
pub mod complex;
use vector::Vector;
use complex::Complex;


#[cfg(test)]
mod tests {
    use crate::vector::VectorKind;

    use super::*;

    // #[test]
    fn linear_algebra() {
        let mut v = Vector::new(vec![2.1, 3.0, 6.0], VectorKind::Row);

        let v2 = Vector::new(vec![-2.0, -3.0, -6.0], VectorKind::Row);
        let a = Vector::angle(&v, &v2);

        println!("{}", v);
        panic!()
    }

    #[test]
    fn complex(){
        println!("{}", Complex{re: 0.0, im: 0.0});
        println!("{}", Complex{re: 1.0, im: 0.0});
        println!("{}", Complex{re: 0.0, im: 1.0});
        println!("{}", Complex{re: 1.0, im: 1.0});
        println!("{}", Complex{re: -1.0, im: -0.0});
        println!("{}", Complex{re: -0.0, im: -1.0});
        println!("{}", Complex{re: -1.0, im: -1.0});

        panic!()
    }
}
