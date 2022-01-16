use core::panic;

use std::ops;

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn unit_vector(vec: Vec3) -> Vec3 {
    vec / vec.length()
}

#[derive(Debug, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
    pub fn from(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn as_color(&self) -> [u8; 3] {
        let r = self.x() * 255.0;
        let g = self.y() * 255.0;
        let b = self.z() * 255.0;

        [r as u8, g as u8, b as u8]
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        *self
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Self::Output {
        Vec3 {
            e: [self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2]],
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Self::Output {
        Vec3 {
            e: [self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2]],
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, v: f64) -> Self::Output {
        Vec3 {
            e: [self.e[0] + v, self.e[1] + v, self.e[2] + v],
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Self::Output {
        self * (1.0 / t)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e = [
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        ];
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2]],
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::from(self * v.x(), self * v.y(), self * v.z())
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: f64) -> Self {
        v * self
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] = self[0] * rhs;
        self[1] = self[1] * rhs;
        self[2] = self[2] * rhs;
    }
}

impl ops::Index<u8> for Vec3 {
    type Output = f64;
    fn index(&self, idx: u8) -> &Self::Output {
        match idx {
            0 => &self.e[0],
            1 => &self.e[1],
            2 => &self.e[2],
            _ => panic!("Index not in range"),
        }
    }
}

impl ops::IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, idx: u8) -> &mut Self::Output {
        match idx {
            0 => &mut self.e[0],
            1 => &mut self.e[1],
            2 => &mut self.e[2],
            _ => panic!("Index not in range"),
        }
    }
}
