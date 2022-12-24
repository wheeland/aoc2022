use num::{Num, Signed};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(PartialEq, Clone, Copy, Default, Hash)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Num + Signed> Vec3<T> {
    pub fn abs(&self) -> Self {
        let x = self.x.abs();
        let y = self.y.abs();
        let z = self.z.abs();
        Self { x, y, z }
    }
    pub fn signum(&self) -> Self {
        let x = self.x.signum();
        let y = self.y.signum();
        let z = self.z.signum();
        Self { x, y, z }
    }
}

impl<T: Num> Add for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Self { x, y, z }
    }
}

impl<T: Num> Add<(T, T, T)> for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: (T, T, T)) -> Self {
        let x = self.x + rhs.0;
        let y = self.y + rhs.1;
        let z = self.z + rhs.2;
        Self { x, y, z }
    }
}

impl<T: Num> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Self { x, y, z }
    }
}

impl<T: Num> Sub<(T, T, T)> for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: (T, T, T)) -> Self {
        let x = self.x - rhs.0;
        let y = self.y - rhs.1;
        let z = self.z - rhs.2;
        Self { x, y, z }
    }
}

impl<T: Num> Mul for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        Self { x, y, z }
    }
}

impl<T: Num> Mul<(T, T, T)> for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: (T, T, T)) -> Self {
        let x = self.x * rhs.0;
        let y = self.y * rhs.1;
        let z = self.z * rhs.2;
        Self { x, y, z }
    }
}

impl<T: Num + Copy> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Self { x, y, z }
    }
}

impl<T: Num + AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Num + AddAssign> AddAssign<(T, T, T)> for Vec3<T> {
    fn add_assign(&mut self, rhs: (T, T, T)) {
        self.x += rhs.0;
        self.y += rhs.1;
        self.z += rhs.2;
    }
}

impl<T: Num + SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Num + SubAssign> SubAssign<(T, T, T)> for Vec3<T> {
    fn sub_assign(&mut self, rhs: (T, T, T)) {
        self.x -= rhs.0;
        self.y -= rhs.1;
        self.z -= rhs.2;
    }
}

impl<T: Num + MulAssign> MulAssign for Vec3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: Num + Copy + MulAssign> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Num + MulAssign> MulAssign<(T, T, T)> for Vec3<T> {
    fn mul_assign(&mut self, rhs: (T, T, T)) {
        self.x *= rhs.0;
        self.y *= rhs.1;
        self.z *= rhs.2;
    }
}

impl<T: Num + Debug> Debug for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Vec3")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

pub type Vec3i32 = Vec3<i32>;
pub type Vec3u32 = Vec3<u32>;
pub type Vec3i64 = Vec3<i64>;
pub type Vec3u64 = Vec3<u64>;
pub type Vec3usize = Vec3<usize>;

pub fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3::new(x, y, z)
}
