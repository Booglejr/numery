use crate::number::{real::Real, imaginary::Imaginary, complex::Complex};


#[test]
fn complex_multiplication_with_real() {
    let result = Complex::new(6.0,3.0) * Real::new(3.0);
    assert_eq!(result, Complex::new(18.0, 9.0))
}

#[test]
fn complex_multiplication_with_imaginary(){
    let result = Complex::new(2.0, 4.0) * Imaginary::new(2.0);
    assert_eq!(result, Complex::new(-8.0, 4.0))
}

#[test]
fn complex_multiplication_with_complex(){
    let result = Complex::new(2.0,3.0) * Complex::new(4.0,6.0);
    assert_eq!(result, Complex::new(-10.0, 24.0))
}