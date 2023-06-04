use std::ops;

use crate::random::{random_f32, random_f32_from_range};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline(always)]
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[inline(always)]
    pub fn random() -> Self {
        Self {
            x: random_f32(),
            y: random_f32(),
            z: random_f32(),
        }
    }

    #[inline(always)]
    pub fn random_in_range(min: f32, max: f32) -> Self {
        Self {
            x: random_f32_from_range(min, max),
            y: random_f32_from_range(min, max),
            z: random_f32_from_range(min, max),
        }
    }

    #[inline(always)]
    pub fn random_in_unit_sphere() -> Self {
        let mut candidate = Self::random_in_range(-1.0, 1.0);
        while candidate.length_squared() >= 1.0 {
            candidate = Self::random_in_range(-1.0, 1.0);
        }
        candidate
    }

    #[inline(always)]
    pub fn random_on_unit_sphere() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    #[inline(always)]
    pub fn of(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline(always)]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline(always)]
    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline(always)]
    pub fn reflect(&self, unit_normal: &Vec3) -> Vec3 {
        self - 2.0f32 * self.dot(unit_normal) * unit_normal
    }

    #[inline(always)]
    pub fn refract(&self, unit_normal: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min((-self).dot(unit_normal), 1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * unit_normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * unit_normal;
        r_out_perp + r_out_parallel
    }

    #[inline(always)]
    pub fn unit(&self) -> Self {
        self / self.length()
    }

    #[inline(always)]
    pub fn near_zero(&self) -> bool {
        return self.x.abs() < 1.0e-8 && self.y.abs() < 1.0e-8 && self.z.abs() < 1.0e-8;
    }
}

impl ops::AddAssign<Vec3> for Vec3 {

    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {

    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f32> for Vec3 {

    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl ops::Mul<Vec3> for &f32 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self.x,
            y: rhs.y * self.y,
            z: rhs.z * self.z,
        }
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Div<&f32> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, rhs: &f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
