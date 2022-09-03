use std::ops::{Add, Div, Mul, Neg, Sub};
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Add for Vec3 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        let add = &rhs.e;
        self.e[0] += add[0];
        self.e[1] += add[1];
        self.e[2] += add[2];
        self
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

    pub fn empty() -> Vec3{
        Vec3::new([0.0;3])
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
            (255.999 * self.x()) as i32,
            (255.999 * self.y()) as i32,
            (255.999 * self.z()) as i32
        )
    }

    pub fn length_squared(&self) -> f64 {
        return self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2];
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }
    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
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
}

pub type Color = Vec3;
pub type Point3 = Vec3;
