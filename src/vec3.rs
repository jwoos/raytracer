use rand;
use std::ops;

use crate::utility;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn random() -> Vec3 {
        Vec3::new(
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>(),
        )
    }

    pub fn random_rng(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            utility::random(min, max),
            utility::random(min, max),
            utility::random(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_rng(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }

            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        return Vec3::random_in_unit_sphere().unit();
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(&in_unit_sphere, &normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3(x, y, z);
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn near_zero(&self) -> bool {
        let s = 1.0e-8;
        return (self.0.abs() < s) && (self.1.abs() < s) && (self.2.abs() < s);
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * Vec3::dot(v, n) * *n
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.0 * v.0 + u.1 * v.1 + u.2 * v.2
    }

    pub fn cross_product(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0,
        )
    }
}

pub type Point = Vec3;
pub type Color = Vec3;

// Unary
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.0, -self.1, -self.2)
    }
}

// Operations with Vec3

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vec3::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Vec3::new(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

// Operations with f64

macro_rules! expr {
    ($e:expr) => {
        $e
    };
}

macro_rules! operations {
    ($vector:ty, $scalar:ty, $trait_for_scalar:ty, $trait_for_vector:ty, $op_fn:ident, $op:tt) => {
        impl $trait_for_scalar for $vector {
            type Output = $vector;

            fn $op_fn(self, other: $scalar) -> Self::Output {
                <$vector>::new(expr!(self.0 $op other), expr!(self.1 $op other), expr!(self.2 $op other))
            }
        }

        impl $trait_for_vector for $scalar {
            type Output = $vector;

            fn $op_fn(self, other: $vector) -> Self::Output {
                <$vector>::new(expr!(other.0 $op self), expr!(other.1 $op self), expr!(other.2 $op self))
            }
        }
    }
}

operations!(Vec3, f64, ops::Add<f64>, ops::Add<Vec3>, add, +);
operations!(Vec3, f64, ops::Sub<f64>, ops::Sub<Vec3>, sub, -);
operations!(Vec3, f64, ops::Mul<f64>, ops::Mul<Vec3>, mul, *);
operations!(Vec3, f64, ops::Div<f64>, ops::Div<Vec3>, div, /);

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        self.0 += other;
        self.1 += other;
        self.2 += other;
    }
}

impl ops::SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        self.0 -= other;
        self.1 -= other;
        self.2 -= other;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}
