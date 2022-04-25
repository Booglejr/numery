use std::ops::{Neg, Add, Sub, Mul, Div};

use super::{real::Real, complex::Complex};


#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Imaginary {
    pub (super) value: f64
}

impl Imaginary{
    pub fn new(value : f64) -> Self{
        Self { value }
    }

    pub fn get_value(self) -> Real {
        Real::new(self.value)
    }
}

///Addition between a Real number and a Real number results in a Real number.
impl Add<Real> for Imaginary{
    type Output = Complex;

    fn add(self, other: Real) -> Complex {
        Complex {
            real: other,
            imaginary: self,
        }
    }
}

///Addition between a Real number and an Imaginary number results in a Complex.
impl Add<Imaginary> for Imaginary{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value
        }
    }
}

///Addition between a Real number and a Complex number results in a Complex number.
impl Add<Complex> for Imaginary{
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: other.real,
            imaginary: self + other.imaginary
        }
    }
}

impl Sub<Real> for Imaginary{
    type Output = Complex;

    fn sub(self, other: Real) -> Complex {
        Complex {
            real: -other,
            imaginary: self
        }
    }
}

impl Sub<Imaginary> for Imaginary{
    type Output = Imaginary;

    fn sub(self, other: Imaginary) -> Imaginary {
        Imaginary {
            value: self.value - other.value
        }
    }
}

impl Sub<Complex> for Imaginary{
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            real: other.real,
            imaginary: self - other.imaginary
        }
    }
}

impl Mul<Real> for Imaginary{
    type Output = Self;

    fn mul(self, other: Real) -> Self{
        Self {
            value: self.value * other.value
        }
    }
}

impl Mul<Imaginary> for Imaginary{
    type Output = Real;

    fn mul(self, other: Imaginary) -> Real {
        Real {
            value: -(self.value * other.value),
        }
    }
}

impl Mul<Complex> for Imaginary{
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self * other.imaginary,
            imaginary: self * other.real
        }
    }
}

impl Div<Real> for Imaginary{
    type Output = Self;

    fn div(self, other: Real) -> Self {
        Self {
            value: self.value / other.value
        }
    }
}

impl Div<Imaginary> for Imaginary{
    type Output = Real;

    fn div(self, other: Imaginary) -> Real {
        Real {
            value: self.value / other.value,
        }
    }
}


impl Div<Complex> for Imaginary{
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let numerator = Complex{
            real: Real::new(0.0),
            imaginary: Imaginary::new(self.value)
        };

        let denominator = other;

        return numerator/denominator;
    }
}

impl Neg for Imaginary {
    type Output = Imaginary;

    fn neg(self) -> Self {
        Self { value: -self.value }
    }
}