use std::f64::consts::PI;

use crate::number::real::Real;

use super::{degree::Degrees, angle::Angle};


#[derive(Clone, Copy)]
pub struct Radians {
    pub (super) value: f64
}

impl Radians {
    fn sin(self) -> Real {
        Real::new(self.value.sin())
    }

    fn cos(self) -> Real {
        Real::new(self.value.cos())
    }   

    fn tan(self) -> Real {
        Real::new(self.value.tan())
    }   
}

impl Angle for Radians {
    fn to_degrees(self) -> Degrees{
        Degrees{
            value: self.value*(180_f64/PI)
        }
    }

    fn to_radians(self) -> Radians{
        self
    }
}