#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3 {
    pub i: f64,
    pub j: f64,
    pub k: f64
}

impl Vec3 {
    pub fn new(i: f64, j: f64, k: f64) -> Vec3 {
        Vec3 { i, j, k }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.i*self.i + self.j*self.j + self.k*self.k
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.i * other.i
        + self.j * other.j
        + self.k * other.k
    }
    
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            i: self.j * other.k - self.k * other.j,
            j: self.k * other.i - self.i * other.k,
            k: self.i * other.j - self.j * other.i
        }
    }
    
    pub fn normalized(&self) -> Vec3 {
        (*self).clone() / self.length()
    }

    pub fn as_rgba(&self) -> [u8; 4] {
        [(self.i * 255.999) as u8, (self.j * 255.999) as u8, (self.k * 255.999) as u8, 0xff]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            i: -self.i,
            j: -self.j,
            k: -self.k
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.i += rhs.i;
        self.j += rhs.j;
        self.k += rhs.k;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k
        }
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.i -= rhs.i;
        self.j -= rhs.j;
        self.k -= rhs.k;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            i: self.i * rhs.i,
            j: self.j * rhs.j,
            k: self.k * rhs.k
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            i: self.i * rhs,
            j: self.j * rhs,
            k: self.k * rhs
        }
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.i *= rhs;
        self.j *= rhs;
        self.k *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            i: self.i / rhs,
            j: self.j / rhs,
            k: self.k / rhs
        }
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.i /= rhs;
        self.j /= rhs;
        self.k /= rhs;
    }
}