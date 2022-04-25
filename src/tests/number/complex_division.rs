use crate::number::{real::Real, imaginary::Imaginary, complex::Complex};


#[test]
fn complex_division_with_real() {
    let result = Complex::new(3.0,6.0) / Real::new(3.0);
    assert_eq!(result,  Complex::new(1.0,2.0))
}

#[test]
fn complex_division_with_imaginary(){
    let result = Complex::new(10.0,5.0) / Imaginary::new(5.0);
    assert_eq!(result, Complex::new(1.0,2.0))
}

#[test]
fn complex_division_with_complex(){
    let result = Complex::new(6.0, 4.0) / Complex::new(2.0,-1.0);
    assert_eq!(result, Complex::new(1.6, 2.8));
}

