use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vec2(pub u16, pub u16);

impl Vec2 {
    pub fn x(&self) -> u16 {
        self.0
    }
    pub fn y(&self) -> u16 {
        self.1
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec2(self.x() + other.x(), self.y() + other.y())
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec2(self.x() - other.x(), self.y() - other.y())
    }
}

impl Div for Vec2 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Vec2(self.x() / other.x(), self.y() / other.y())
    }
}

impl Mul for Vec2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec2(self.x() * rhs.x(), self.y() * rhs.y())
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
    }
}
