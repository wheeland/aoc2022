use core::fmt::Debug;
use std::slice::Iter;
use std::slice::IterMut;

use crate::vec2::Vec2i32;

#[derive(Clone)]
pub struct Array2D<T: Default + Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Default + Clone> Array2D<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![T::default(); width * height];
        Self {
            width,
            height,
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn slice(&self, y: usize) -> &[T] {
        let start = y * self.width;
        let end = (y + 1) * self.width;
        &self.data[start..end]
    }

    pub fn slice_mut(&mut self, y: usize) -> &mut [T] {
        let start = y * self.width;
        let end = (y + 1) * self.width;
        &mut self.data[start..end]
    }

    pub fn fill(&mut self, value: T) {
        for v in &mut self.data {
            *v = value.clone();
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }
}

pub trait Array2DIndex {
    fn coords(&self) -> (usize, usize);
}

impl Array2DIndex for (usize, usize) {
    fn coords(&self) -> (usize, usize) {
        *self
    }
}

impl Array2DIndex for (i32, i32) {
    fn coords(&self) -> (usize, usize) {
        (self.0 as usize, self.1 as usize)
    }
}

impl Array2DIndex for Vec2i32 {
    fn coords(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

impl<T: Default + Clone> Array2D<T> {
    pub fn at<P: Array2DIndex>(&self, pos: P) -> &T {
        let coords = pos.coords();
        debug_assert!(coords.0 < self.width);
        debug_assert!(coords.1 < self.height);
        &self.data[coords.0 + self.width * coords.1]
    }

    pub fn at_mut<P: Array2DIndex>(&mut self, pos: P) -> &mut T {
        let coords = pos.coords();
        debug_assert!(coords.0 < self.width);
        debug_assert!(coords.1 < self.height);
        &mut self.data[coords.0 + self.width * coords.1]
    }

    pub fn set<P: Array2DIndex>(&mut self, pos: P, value: T) {
        *self.at_mut(pos) = value;
    }
}

impl<T: Default + Debug + Clone> Debug for Array2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for y in 0..self.height {
            let c1 = y * self.width;
            let c2 = (y + 1) * self.width;
            let dbg: Vec<&T> = self.data[c1..c2].iter().collect();
            list.entry(&dbg);
        }
        list.finish()
    }
}
