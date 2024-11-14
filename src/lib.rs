pub mod vector;
pub mod complex;
use vector::Vector;
use complex::Complex;


#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{complex::ComplexForm, vector::VectorKind};

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
        // formatting test
        let one = Complex{re: 1.0, im: 0.0, form: ComplexForm::Alg};
        let i = Complex{re: 0.0, im: 1.0, form: ComplexForm::Alg};
        let oneplusi = Complex{re: 1.0, im: 1.0, form: ComplexForm::Alg};
        assert_eq!(format!("{}", one - one), "0");
        assert_eq!(format!("{}", one), "1");
        assert_eq!(format!("{}", -one), "-1");
        assert_eq!(format!("{}", i), "i");
        assert_eq!(format!("{}", -i), "-i");
        assert_eq!(format!("{}", oneplusi), "1+i");
        assert_eq!(format!("{}", -oneplusi), "-1-i");

        // abs test
        let c = Complex{ re: 3.0, im: 4.0, form: ComplexForm::Alg };
        assert_eq!(c.abs(), 5.0);

        // arg test
        let c = Complex::i();
        assert_eq!(c.arg(), PI / 2.0);
        let c = Complex::from(1);
        assert_eq!(c.arg(), 0.0);

        // conjugate test
        let c1 = Complex{ re: 1.0, im: 1.0, form: ComplexForm::Alg };
        let c2 = Complex{ re: 1.0, im: -1.0, form: ComplexForm::Alg };
        assert_eq!(c1.conjugate(), c2);
        assert_eq!(c2.conjugate(), c1);

    }
}
