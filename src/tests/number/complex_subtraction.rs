use crate::number::{real::Real, imaginary::Imaginary, complex::Complex};


#[test]
fn complex_subtraction_with_real() {
    let result = Complex::new(4.0,2.0) - Real::new(3.0);
    assert_eq!(result, Complex::new(1.0,2.0))
}

#[test]
fn complex_subtraction_with_imaginary(){
    let result = Complex::new(6.0,2.0) - Imaginary::new(7.0);
    assert_eq!(result, Complex::new(6.0,-5.0))
}

#[test]
fn complex_subtraction_with_negative_imaginary(){
    let result = Complex::new(6.0,7.0) - Imaginary::new(-7.0);
    assert_eq!(result,  Complex::new(6.0,14.0))
}

#[test]
fn complex_subtraction_with_complex(){
    let result = Complex::new(9.0,10.0) - Complex::new(7.0,-5.0);
    assert_eq!(result, Complex::new(2.0, 15.0))
}