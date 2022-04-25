use std::ops::{Div, Add, Sub, Mul};

use crate::angles::angle::Angle;
use crate::number::imaginary::Imaginary;
use crate::number::real::Real;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Complex {
    pub (super) real: Real,
    pub (super) imaginary: Imaginary
}

impl Complex{
    pub fn new(real_value: f64, imaginary_value: f64) -> Self{
        Self {
            real: Real::new(real_value),
            imaginary: Imaginary::new(imaginary_value)
        }
    }

    pub fn cis(angle:impl Angle) -> Self {
        Self{
            real: Real::new(angle.cos().into()),
            imaginary: Imaginary::new(angle.sin().into()),
        }
    }

    pub fn get_real(self) -> Real {
        self.real
    }

    pub fn get_imaginary(self) -> Imaginary {
        self.imaginary
    }

    pub fn conjugate(self) -> Self {
        Self {
            real: self.real,
            imaginary: -self.imaginary
        }
    }
}

impl Add<Real> for Complex {
    type Output = Complex;

    fn add(self, other: Real) -> Complex{
        Complex{
            real: self.real + other,
            imaginary: self.imaginary
        }
    }
}

impl Add<Imaginary> for Complex {
    type Output = Complex;

    fn add(self, other: Imaginary) -> Complex{
        Complex{
            real: self.real,
            imaginary: self.imaginary + other
        }
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex{
        Complex{
            real: self.real+other.real,
            imaginary: self.imaginary + other.imaginary
        }
    }
}

impl Sub<Real> for Complex {
    type Output = Complex;

    fn sub(self, other: Real) -> Complex{
        Complex{
            real: self.real - other,
            imaginary: self.imaginary
        }
    }
}

impl Sub<Imaginary> for Complex {
    type Output = Complex;

    fn sub(self, other: Imaginary) -> Complex{
        Complex{
            real: self.real,
            imaginary: self.imaginary - other
        }
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex{
        Complex{
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary
        }
    }
}

impl Mul<Real> for Complex{
    type Output = Self;

    fn mul(self, other: Real) -> Self{
        Self {
            real: self.real * other,
            imaginary: self.imaginary * other
        }
    }
}

impl Mul<Imaginary> for Complex{
    type Output = Complex;

    fn mul(self, other: Imaginary) -> Complex {
        Complex {
            real: self.imaginary * other,
            imaginary: self.real * other
        }
    }
}

impl Mul<Complex> for Complex{
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        let a = self.get_real();
        let b = self.get_imaginary().get_value();

        let c = other.get_real();
        let d = other.get_imaginary().get_value();

        Complex{
            real: (a*c - b*d),
            imaginary: Imaginary::new((a*d+b*c).into())           
        }
    }
}

impl Div<Real> for Complex{
    type Output = Self;

    fn div(self, other: Real) -> Self {
        Self {
            real: self.real / other,
            imaginary: self.imaginary / other
        }
    }
}

impl Div<Imaginary> for Complex{
    type Output = Complex;

    fn div(self, other: Imaginary) -> Self {
        Self {
            real: self.imaginary / other,
            imaginary: self.real / other
        }
    }
}

impl Div<Complex> for Complex{
    type Output = Complex;
    
    fn div(self, other: Complex) -> Complex {
        let a = self.get_real();
        let b = self.get_imaginary().get_value();

        let c = other.get_real();
        let d = other.get_imaginary().get_value();

        Complex{
            real: (a*c + b*d)/(c*c + d*d),
            imaginary: Imaginary::new(((b*c - a*d)/(c*c + d*d)).into())           
        }
    }

}