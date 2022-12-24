use num::{Num, Signed};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Rem, Sub, SubAssign},
};

#[derive(PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(v: (T, T)) -> Self {
        Self::new(v.0, v.1)
    }
}

impl<T> Into<(T, T)> for Vec2<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

trait VecBoolTestable {
    fn test(&self) -> bool;
}
impl VecBoolTestable for bool {
    fn test(&self) -> bool {
        *self
    }
}

impl<T: Num + Ord + Copy> Vec2<T> {
    pub fn min_element(&self) -> T {
        self.x.min(self.y)
    }
    pub fn max_element(&self) -> T {
        self.x.max(self.y)
    }
    pub fn min(&self, rhs: Self) -> Self {
        let x = self.x.min(rhs.x);
        let y = self.y.min(rhs.y);
        Self { x, y }
    }
    pub fn max(&self, rhs: Self) -> Self {
        let x = self.x.max(rhs.x);
        let y = self.y.max(rhs.y);
        Self { x, y }
    }
}

impl<T: VecBoolTestable> Vec2<T> {
    pub fn all(&self) -> bool {
        self.x.test() && self.y.test()
    }
    pub fn any(&self) -> bool {
        self.x.test() || self.y.test()
    }
    pub fn none(&self) -> bool {
        !self.all()
    }
}

impl<T: Num + Ord> Vec2<T> {
    pub fn lt(self, other: Vec2<T>) -> Vec2<bool> {
        vec2(self.x.lt(&other.x), self.y.lt(&other.y))
    }
    pub fn le(self, other: Vec2<T>) -> Vec2<bool> {
        vec2(self.x.le(&other.x), self.y.le(&other.y))
    }
    pub fn gt(self, other: Vec2<T>) -> Vec2<bool> {
        vec2(self.x.gt(&other.x), self.y.gt(&other.y))
    }
    pub fn ge(self, other: Vec2<T>) -> Vec2<bool> {
        vec2(self.x.ge(&other.x), self.y.ge(&other.y))
    }
}

impl<T: Num + Signed> Vec2<T> {
    pub fn abs(&self) -> Self {
        let x = self.x.abs();
        let y = self.y.abs();
        Self { x, y }
    }
    pub fn signum(&self) -> Self {
        let x = self.x.signum();
        let y = self.y.signum();
        Self { x, y }
    }
}

impl<T: Num> Add for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Self { x, y }
    }
}

impl<T: Num> Add<(T, T)> for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: (T, T)) -> Self {
        let x = self.x + rhs.0;
        let y = self.y + rhs.1;
        Self { x, y }
    }
}

impl<T: Num> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Self { x, y }
    }
}

impl<T: Num> Sub<(T, T)> for Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: (T, T)) -> Self {
        let x = self.x - rhs.0;
        let y = self.y - rhs.1;
        Self { x, y }
    }
}

impl<T: Num> Mul for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        Self { x, y }
    }
}

impl<T: Num> Mul<(T, T)> for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: (T, T)) -> Self {
        let x = self.x * rhs.0;
        let y = self.y * rhs.1;
        Self { x, y }
    }
}

impl<T: Num + Copy> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let x = self.x * rhs;
        let y = self.y * rhs;
        Self { x, y }
    }
}

impl<T: Num> Div for Vec2<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        Self { x, y }
    }
}

impl<T: Num + Copy> Div<T> for Vec2<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let x = self.x / rhs;
        let y = self.y / rhs;
        Self { x, y }
    }
}

impl<T: Num> Rem for Vec2<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        let x = self.x % rhs.x;
        let y = self.y % rhs.y;
        Self { x, y }
    }
}

impl<T: Num + AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Num + AddAssign> AddAssign<(T, T)> for Vec2<T> {
    fn add_assign(&mut self, rhs: (T, T)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl<T: Num + SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Num + SubAssign> SubAssign<(T, T)> for Vec2<T> {
    fn sub_assign(&mut self, rhs: (T, T)) {
        self.x -= rhs.0;
        self.y -= rhs.1;
    }
}

impl<T: Num + MulAssign> MulAssign for Vec2<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Num + Copy + MulAssign> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Num + MulAssign> MulAssign<(T, T)> for Vec2<T> {
    fn mul_assign(&mut self, rhs: (T, T)) {
        self.x *= rhs.0;
        self.y *= rhs.1;
    }
}

impl<T: Num + Debug> Debug for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Vec2").field(&self.x).field(&self.y).finish()
    }
}

pub type Vec2i32 = Vec2<i32>;
pub type Vec2u32 = Vec2<u32>;
pub type Vec2i64 = Vec2<i64>;
pub type Vec2u64 = Vec2<u64>;
pub type Vec2usize = Vec2<usize>;

pub fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2::new(x, y)
}
