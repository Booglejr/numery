use std::{ops::{Add, Mul, Sub, Div, Neg}};

use super::{imaginary::Imaginary, complex::Complex};


#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Real{
    pub (super) value: f64
}


///Represents real numbers and can be considered equivalent to an f64.
impl Real {
    pub fn new(value : f64) -> Self{
        Self { value }
    }
}


///Addition between a Real number and a Real number results in a Real number.
impl Add<Real> for Real{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value
        }
    }
}

///Addition between a Real number and an Imaginary number results in a Complex.
impl Add<Imaginary> for Real{
    type Output = Complex;

    fn add(self, other: Imaginary) -> Complex {
        Complex {
            real: self,
            imaginary: other
        }
    }
}

///Addition between a Real number and a Complex number results in a Complex number.
impl Add<Complex> for Real{
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self + other.real,
            imaginary: other.imaginary
        }
    }
}

impl Sub<Real> for Real{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value
        }
    }
}

impl Sub<Imaginary> for Real{
    type Output = Complex;

    fn sub(self, other: Imaginary) -> Complex {
        Complex {
            real: self,
            imaginary: -other
        }
    }
}

impl Sub<Complex> for Real{
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            real: self - other.real,
            imaginary: -other.imaginary
        }
    }
}

impl Mul<Real> for Real{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            value: self.value * other.value
        }
    }
}

impl Mul<Imaginary> for Real{
    type Output = Imaginary;

    fn mul(self, other: Imaginary) -> Imaginary {
        Imaginary {
            value: self.value * other.value,
        }
    }
}

impl Mul<Complex> for Real{
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self * other.real,
            imaginary: self * other.imaginary
        }
    }
}

impl Div<Real> for Real{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            value: self.value / other.value
        }
    }
}

impl Div<Imaginary> for Real{
    type Output = Imaginary;

    fn div(self, other: Imaginary) -> Imaginary {
        Imaginary {
            value: self.value / other.value,
        }
    }
}


impl Div<Complex> for Real{
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let numerator = Complex{
            real: Real::new(self.value),
            imaginary: Imaginary::new(0.0)
        };

        let denominator = other;

        return numerator/denominator;
    }
}


impl Neg for Real {
    type Output = Real;

    fn neg(self) -> Self {
        Self { value: -self.value }
    }
}

impl std::convert::Into<f64> for Real {
    fn into(self) -> f64 {
        return self.value;
    }
}