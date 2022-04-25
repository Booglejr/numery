use crate::number::{real::Real, imaginary::Imaginary, complex::Complex};


#[test]
fn imaginary_division_with_real() {
    let result = Imaginary::new(3.0) / Real::new(3.0);
    assert_eq!(result, Imaginary::new(1.0))
}

#[test]
fn imaginary_division_with_imaginary(){
    let result = Imaginary::new(10.0) / Imaginary::new(5.0);
    assert_eq!(result, Real::new(2.0))
}

#[test]
fn imaginary_division_with_complex(){
    let result = Imaginary::new(-3.0) / Complex::new(2.0,-1.0);
    assert_eq!(result, Complex::new(0.6, -1.2))
}

