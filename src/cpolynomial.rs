//CPolynomial is a complex-valued polynomial with real (f64) coefficients

// To-do: add in all four operations for CPolynomial and corresponding tests in op_test
// figure out powers and stuff

use complex::Complex;
use std::vec::Vec;
// the first element of the vector should be the constant term!
struct CPolynomial {
    coeff: Vec<f64>
}

impl CPolynomial {
    fn new(v: Vec<f64>) -> CPolynomial {
        CPolynomial { coeff: v }
    }
    fn eval(&self, x: Complex<f64>) -> Complex<f64> {
        let mut c = Complex::new(0f64, 0f64);
        let mut deg = 0;
        for &i in self.coeff.iter() {
            c = c + x.pow(deg).scale(i); // this is clearly wrong!
            deg += 1;
        }
        c
    }
    fn add(&self, other: CPolynomial) -> CPolynomial {
        let mut new_coeff: Vec<f64> = Vec::new();
        if self.coeff.len() < other.coeff.len() {
            for i in range(0u, self.coeff.len()) {
                new_coeff.push(self.coeff[i] + other.coeff[i]);
            }
            for i in range(self.coeff.len(), other.coeff.len()) {
                new_coeff.push(other.coeff[i]);
            }
        }
        else {
            for i in range(0u, other.coeff.len()) {
                new_coeff.push(self.coeff[i] + other.coeff[i]);
            }
            for i in range(other.coeff.len(), self.coeff.len()) {
                new_coeff.push(self.coeff[i]);
            }
        }
        CPolynomial::new(new_coeff)
    }
}

#[cfg(test)]
mod test {
    use complex::Complex;
    use cpolynomial::CPolynomial;
    use std::vec::Vec;

    #[test]
    fn eval_test() {
        let p1 = CPolynomial::new(vec!(0f64, 1f64, 2f64, 7f64));
        let p2 = CPolynomial::new(vec!(4f64, 4f64, 4f64, 3f64));
        assert_eq!(p1.eval(Complex::new(2.5f64, 0f64)).re, 124.375f64);
        assert_eq!(p1.eval(Complex::new(2.5f64, 0f64)).im, 0f64);
        assert_eq!(p2.eval(Complex::new(1f64, -2f64)).re, -37f64);
        assert_eq!(p2.eval(Complex::new(1f64, -2f64)).im, -18f64);
    }

    // need to include subtract, divide, multiply
    #[test]
    fn op_test() {
        let p1 = CPolynomial::new(vec!(0.0f64, 1.0, 2.0, 7.0));
        let p2 = CPolynomial::new(vec!(4.0f64, 4.0, 4.0, 3.0));
        assert_eq!(p1.add(p2).coeff, vec!(4.0f64, 5.0, 6.0, 10.0));
    }
}
