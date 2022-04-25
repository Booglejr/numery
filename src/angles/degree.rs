use std::f64::consts::PI;

use super::{radian::Radians, angle::Angle};

#[derive(Clone, Copy)]
pub struct Degrees{
    pub (super) value: f64
}

impl Angle for Degrees {
    fn to_degrees(self) -> Degrees{
        self
    }

    fn to_radians(self) -> Radians{
        Radians{
            value: self.value*(PI/180_f64)
        }
    }
}