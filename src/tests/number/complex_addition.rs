use crate::number::{real::Real, imaginary::Imaginary, complex::Complex};


#[test]
fn complex_addition_with_real() {
    let result = Complex::new(4.0,6.0) + Real::new(3.0);
    assert_eq!(result, Complex::new(7.0,6.0))
}

#[test]
fn complex_addition_with_imaginary(){
    let result = Complex::new(4.0,6.0) + Imaginary::new(7.0);
    assert_eq!(result, Complex::new(4.0,13.0))
}

#[test]
fn complex_addition_with_complex(){
    let result = Complex::new(4.0,6.0) + Complex::new(7.0,-5.0);
    assert_eq!(result, Complex::new(11.0, 1.0))
}

