use std::ops::{
    Add,
    Mul,
    Sub,
    Div
};

pub struct Point3D<T> {
    pub(crate) x: T,
    pub(crate) y: T,
    pub(crate) z: T
}

impl <T> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Point3D<T> {
        Point3D {
            x,
            y,
            z
        }
    }
}

impl <T: std::ops::Sub<Output = T>> Sub for Point3D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }

    }
}

impl <T: std::ops::Add<Output = T>> Add for Point3D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl <T: std::ops::Mul<Output = T>> Mul for Point3D<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl <T: std::ops::Div<Output = T>> Div for Point3D<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

