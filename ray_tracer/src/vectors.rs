
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            e: [x, y, z]
        }
    }
    pub fn from_color(r: f64, g: f64, b: f64) -> Self {
        Self {
            e: [r, g, b]
        }
    }
    pub fn r(&self) -> f64 {
        return self.e[0];
    }
    pub fn g(&self) -> f64 {
        return self.e[1];
    }
    pub fn b(&self) -> f64 {
        return self.e[2];
    }
    pub fn x(&self) -> f64 {
        return self.e[0];
    }
    pub fn y(&self) -> f64 {
        return self.e[1];
    }
    pub fn z(&self) -> f64 {
        return self.e[2];
    }
    pub fn length(&self) -> f64 {
        (self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]).sqrt()
    }
    pub fn squared_length(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }
    pub fn unit_vector(&self) -> Self {
        let length = self.length();
        return *self/length;
    }
    pub fn gamma2_on_color(&self) -> Self {
        return Self {
            e: [self.e[0].sqrt(), self.e[1].sqrt(), self.e[2].sqrt()]
        };
    }
    pub fn to_string(&self) -> String {
        return "{ ".to_string() 
        + self.e[0].to_string().as_str() + ", "
        + self.e[1].to_string().as_str() + ", "
        + self.e[2].to_string().as_str() + " }";
    }
}

// Same as a • b = |a||b|cosθ, where θ is the angle between the two vectors
pub fn dot(a: Vec3, b: Vec3) -> f64 {
    return a.e[0] * b.e[0] + a.e[1] * b.e[1] + a.e[2] * b.e[2];
}

use std::{ops};
impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            e: [self.e[0] + rhs.e[0], 
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2]]
        }
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            e: [self.e[0] - rhs.e[0], 
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2]]
        }
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            e: [self.e[0] * rhs.e[0], 
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2]]
        }
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            e: [self.e[0] * rhs, 
            self.e[1] * rhs,
            self.e[2] * rhs]
        }
    }
}
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [rhs.e[0] * self, 
            rhs.e[1] * self,
            rhs.e[2] * self]
        }
    }
}
impl ops::Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            e: [self.e[0] / rhs.e[0], 
            self.e[1] / rhs.e[1],
            self.e[2] / rhs.e[2]]
        }
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            e: [self.e[0] / rhs, 
            self.e[1] / rhs,
            self.e[2] / rhs]
        }
    }
}
impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.e[index];
    }
}
impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.e[index];
    }
}