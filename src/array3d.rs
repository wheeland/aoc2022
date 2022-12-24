use core::fmt::Debug;
use std::slice::Iter;
use std::slice::IterMut;

use crate::vec3::Vec3i32;

#[derive(Clone)]
pub struct Array3D<T: Default + Clone> {
    width: usize,
    height: usize,
    depth: usize,
    data: Vec<T>,
}

impl<T: Default + Clone> Array3D<T> {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        let data = vec![T::default(); width * height * depth];
        Self {
            width,
            height,
            depth,
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn depth(&self) -> usize {
        self.depth
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

pub trait Array3DIndex {
    fn coords(&self) -> (usize, usize, usize);
}

impl Array3DIndex for (usize, usize, usize) {
    fn coords(&self) -> (usize, usize, usize) {
        *self
    }
}

impl Array3DIndex for (i32, i32, i32) {
    fn coords(&self) -> (usize, usize, usize) {
        (self.0 as usize, self.1 as usize, self.2 as usize)
    }
}

impl Array3DIndex for Vec3i32 {
    fn coords(&self) -> (usize, usize, usize) {
        (self.x as usize, self.y as usize, self.z as usize)
    }
}

impl<T: Default + Clone> Array3D<T> {
    pub fn at<P: Array3DIndex>(&self, pos: P) -> &T {
        let coords = pos.coords();
        debug_assert!(coords.0 < self.width);
        debug_assert!(coords.1 < self.height);
        debug_assert!(coords.2 < self.depth);
        &self.data[coords.0 + self.width * coords.1 + self.width * self.height * coords.2]
    }

    pub fn at_mut<P: Array3DIndex>(&mut self, pos: P) -> &mut T {
        let coords = pos.coords();
        debug_assert!(coords.0 < self.width);
        debug_assert!(coords.1 < self.height);
        debug_assert!(coords.2 < self.depth);
        &mut self.data[coords.0 + self.width * coords.1 + self.width * self.height * coords.2]
    }

    pub fn set<P: Array3DIndex>(&mut self, pos: P, value: T) {
        *self.at_mut(pos) = value;
    }
}
