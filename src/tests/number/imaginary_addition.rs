use crate::number::{real::Real, imaginary::Imaginary, complex::Complex};


#[test]
fn imaginary_addition_with_real() {
    let result = Imaginary::new(4.0) + Real::new(3.0);
    assert_eq!(result, Complex::new(3.0,4.0))
}

#[test]
fn imaginary_addition_with_imaginary(){
    let result = Imaginary::new(6.0) + Imaginary::new(7.0);
    assert_eq!(result, Imaginary::new(13.0))
}

#[test]
fn imaginary_addition_with_complex(){
    let result = Imaginary::new(6.0) + Complex::new(7.0,-5.0);
    assert_eq!(result, Complex::new(7.0, 1.0))
}

