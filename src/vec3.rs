use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 (
    f64, f64, f64
);

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3(x, y, z);
    }

    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> f64 {
        self.2
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 + self.1 + self.2 * self.2
    }

    fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.0 * v.0 + u.1 * v.1 + u.2 * v.2
    }

    fn cross_product(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0,
            )
    }

    fn unit(&self) -> Vec3 {
        *self / self.length()
    }
}

// Operations with Vec3

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3::new(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
        )
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
        Vec3::new(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
        )
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
        Vec3::new(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
        )
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
        Vec3::new(
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2,
        )
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

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f64) -> Self::Output {
        Vec3::new(
            self.0 + other,
            self.1 + other,
            self.2 + other,
        )
    }
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        self.0 += other;
        self.1 += other;
        self.2 += other;
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f64) -> Self::Output {
        Vec3::new(
            self.0 - other,
            self.1 - other,
            self.2 - other,
        )
    }
}

impl ops::SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        self.0 -= other;
        self.1 -= other;
        self.2 -= other;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Self::Output {
        Vec3::new(
            self.0 * other,
            self.1 * other,
            self.2 * other,
        )
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        Vec3::new(
            self.0 / other,
            self.1 / other,
            self.2 / other,
        )
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}
