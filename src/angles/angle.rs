use crate::{angles::{degree::Degrees, radian::Radians}, number::real::Real};


pub trait Angle: Copy + Clone{
    fn to_degrees(self) -> Degrees;

    fn to_radians(self) -> Radians;

    fn sin(&self) -> Real{
        self.to_radians().sin()
    }

    fn cos(&self) -> Real {
        self.to_radians().cos()
    }

    fn tan(&self) -> Real {
        self.to_radians().tan()
    }
}