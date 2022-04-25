use crate::number::{real::Real, imaginary::Imaginary, complex::Complex};


#[test]
fn real_subtraction_with_real() {
    let result = Real::new(4.0) - Real::new(3.0);
    assert_eq!(result, Real::new(1.0))
}

#[test]
fn real_subtraction_with_imaginary(){
    let result = Real::new(6.0) - Imaginary::new(7.0);
    assert_eq!(result, Complex::new(6.0, -7.0))
}

#[test]
fn real_subtraction_with_negative_imaginary(){
    let result = Real::new(6.0) - Imaginary::new(-7.0);
    assert_eq!(result, Complex::new(6.0, 7.0))
}

#[test]
fn real_subtraction_with_complex(){
    let result = Real::new(6.0) - Complex::new(7.0,-5.0);
    assert_eq!(result, Complex::new(-1.0, 5.0))
}