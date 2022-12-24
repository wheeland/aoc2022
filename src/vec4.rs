use num::{Num, Signed};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign},
};

use crate::{vec2::Vec2, vec3::Vec3};

#[derive(PartialEq, Clone, Copy, Default, Hash)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Copy> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn xy(&self) -> Vec2<T> {
        Vec2::new(self.x, self.y)
    }

    pub fn xyz(&self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn xz(&self) -> Vec2<T> {
        Vec2::new(self.x, self.z)
    }
}

impl<T: Num + Ord + Copy> Vec4<T> {
    pub fn min_element(&self) -> T {
        self.x.min(self.y).min(self.z).min(self.w)
    }
    pub fn max_element(&self) -> T {
        self.x.max(self.y).max(self.z).max(self.w)
    }
}

impl<T: Num + Ord> Vec4<T> {
    pub fn lt(self, other: Vec4<T>) -> Vec4<bool> {
        vec4(
            self.x.lt(&other.x),
            self.y.lt(&other.y),
            self.z.lt(&other.z),
            self.w.lt(&other.w),
        )
    }
    pub fn le(self, other: Vec4<T>) -> Vec4<bool> {
        vec4(
            self.x.le(&other.x),
            self.y.le(&other.y),
            self.z.le(&other.z),
            self.w.le(&other.w),
        )
    }
    pub fn gt(self, other: Vec4<T>) -> Vec4<bool> {
        vec4(
            self.x.gt(&other.x),
            self.y.gt(&other.y),
            self.z.gt(&other.z),
            self.w.gt(&other.w),
        )
    }
    pub fn ge(self, other: Vec4<T>) -> Vec4<bool> {
        vec4(
            self.x.ge(&other.x),
            self.y.ge(&other.y),
            self.z.ge(&other.z),
            self.w.ge(&other.w),
        )
    }
}

impl<T: Num + Signed> Vec4<T> {
    pub fn abs(&self) -> Self {
        let x = self.x.abs();
        let y = self.y.abs();
        let z = self.z.abs();
        let w = self.w.abs();
        Self { x, y, z, w }
    }
    pub fn signum(&self) -> Self {
        let x = self.x.signum();
        let y = self.y.signum();
        let z = self.z.signum();
        let w = self.w.signum();
        Self { x, y, z, w }
    }
}

impl<T: Num> Add for Vec4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        let w = self.w + rhs.w;
        Self { x, y, z, w }
    }
}

impl<T: Num> Add<(T, T, T, T)> for Vec4<T> {
    type Output = Self;
    fn add(self, rhs: (T, T, T, T)) -> Self {
        let x = self.x + rhs.0;
        let y = self.y + rhs.1;
        let z = self.z + rhs.2;
        let w = self.w + rhs.3;
        Self { x, y, z, w }
    }
}

impl<T: Num> Sub for Vec4<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        let w = self.w - rhs.w;
        Self { x, y, z, w }
    }
}

impl<T: Num> Sub<(T, T, T, T)> for Vec4<T> {
    type Output = Self;
    fn sub(self, rhs: (T, T, T, T)) -> Self {
        let x = self.x - rhs.0;
        let y = self.y - rhs.1;
        let z = self.z - rhs.2;
        let w = self.w - rhs.3;
        Self { x, y, z, w }
    }
}

impl<T: Num> Mul for Vec4<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        let w = self.w * rhs.w;
        Self { x, y, z, w }
    }
}

impl<T: Num> Mul<(T, T, T, T)> for Vec4<T> {
    type Output = Self;
    fn mul(self, rhs: (T, T, T, T)) -> Self {
        let x = self.x * rhs.0;
        let y = self.y * rhs.1;
        let z = self.z * rhs.2;
        let w = self.w * rhs.3;
        Self { x, y, z, w }
    }
}

impl<T: Num + Copy> Mul<T> for Vec4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        let w = self.w * rhs;
        Self { x, y, z, w }
    }
}

impl<T: Num> Div for Vec4<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        let z = self.z / rhs.z;
        let w = self.w / rhs.w;
        Self { x, y, z, w }
    }
}

impl<T: Num + AddAssign> AddAssign for Vec4<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl<T: Num + AddAssign> AddAssign<(T, T, T, T)> for Vec4<T> {
    fn add_assign(&mut self, rhs: (T, T, T, T)) {
        self.x += rhs.0;
        self.y += rhs.1;
        self.z += rhs.2;
        self.w += rhs.3;
    }
}

impl<T: Num + SubAssign> SubAssign for Vec4<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl<T: Num + SubAssign> SubAssign<(T, T, T, T)> for Vec4<T> {
    fn sub_assign(&mut self, rhs: (T, T, T, T)) {
        self.x -= rhs.0;
        self.y -= rhs.1;
        self.z -= rhs.2;
        self.w -= rhs.3;
    }
}

impl<T: Num + MulAssign> MulAssign for Vec4<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl<T: Num + Copy + MulAssign> MulAssign<T> for Vec4<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl<T: Num + MulAssign> MulAssign<(T, T, T, T)> for Vec4<T> {
    fn mul_assign(&mut self, rhs: (T, T, T, T)) {
        self.x *= rhs.0;
        self.y *= rhs.1;
        self.z *= rhs.2;
        self.w *= rhs.3;
    }
}

impl<T: Num + Debug> Debug for Vec4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Vec4")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

pub type Vec4i32 = Vec4<i32>;
pub type Vec4u32 = Vec4<u32>;
pub type Vec4i64 = Vec4<i64>;
pub type Vec4u64 = Vec4<u64>;
pub type Vec4usize = Vec4<usize>;

pub fn vec4<T: Copy>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4::new(x, y, z, w)
}
