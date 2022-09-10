use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

use super::mathf64::{clamp, random_f64, random_f64_01};
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Add for Vec3 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
        self
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self {
        let sub = &rhs.e;
        self.e[0] -= sub[0];
        self.e[1] -= sub[1];
        self.e[2] -= sub[2];
        self
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new([
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        ])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
        self
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
        self
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        rhs / self
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.e[0] = -self.e[0];
        self.e[1] = -self.e[1];
        self.e[2] = -self.e[2];
        self
    }
}

///Constructor
impl Vec3 {
    pub fn new(array: [f64; 3]) -> Vec3 {
        return Vec3 { e: array };
    }

    pub fn empty() -> Vec3 {
        Vec3::new([0.0; 3])
    }
}

///Get value
impl Vec3 {
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }
}

///Func
impl Vec3 {
    pub fn to_r8g8b8_string(&self) -> String {
        format!(
            "{} {} {}\n",
            (256.0 * clamp(self.x().sqrt(), 0.0, 0.999)) as i32,
            (256.0 * clamp(self.y().sqrt(), 0.0, 0.999)) as i32,
            (256.0 * clamp(self.z().sqrt(), 0.0, 0.999)) as i32
        )
    }

    pub fn length_squared(&self) -> f64 {
        return self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }
    pub fn unit_vector(v: Vec3) -> Vec3 {
        let length = v.length();
        if length == 0.0 {
            v
        } else {
            v / length
        }
    }
    pub fn dot(u: Vec3, v: Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3::new([
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        ])
    }

    pub fn random() -> Vec3 {
        Vec3::new([random_f64_01(), random_f64_01(), random_f64_01()])
    }
    pub fn random_min_max(min: f64, max: f64) -> Vec3 {
        Vec3::new([
            random_f64(min, max),
            random_f64(min, max),
            random_f64(min, max),
        ])
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_min_max(-1.0, 1.0);
            //这里的1很显然是1的平方
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::unit_vector(Self::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        //
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(in_unit_sphere, normal) >= 0.0 {
            return in_unit_sphere;
        }
        return -in_unit_sphere;
    }

    pub fn ramdom_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new([random_f64(-1., 1.), random_f64(-1., 1.), 0.]);
            if p.length_squared() >= 1. {
                continue;
            }
            return p;
        }
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - ((2.0 * Vec3::dot(*self, *normal)) * *normal)
    }

    pub fn refract(&self, &normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(-*self, normal).min(1.0);

        let r_out_prep = etai_over_etat * (*self + cos_theta * normal);

        let r_out_parallel = -(1.0 - r_out_prep.length_squared()).abs().sqrt() * normal;

        let refr = r_out_prep + r_out_parallel;

        return refr;
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < 0.0 && self.e[1].abs() < 0.0 && self.e[2].abs() < 0.0
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;
