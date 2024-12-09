use crate::vec2::{Dir, Vec2i};
use std::ops::{Index, IndexMut};

/// 2D map type.
#[derive(Debug, Clone)]
pub struct Map<T> {
    pub h: usize,
    pub w: usize,
    data: Vec<T>, // row-major
}

impl<T: Clone> Map<T> {
    /// Create map from 2d matrix.
    pub fn from_vecs(vecs: Vec<Vec<T>>) -> Map<T> {
        let h = vecs.len();
        let w = vecs[0].len();
        let data: Vec<_> = vecs.into_iter().flatten().collect();
        assert_eq!(data.len(), h * w);
        Map { h, w, data }
    }

    /// Move in direction within map.
    pub fn step_within(&self, pos: &Vec2i, dir: &Dir, d: isize) -> Option<Vec2i> {
        let new = pos.step(dir, d);
        if self.contains(&new) {
            Some(new)
        } else {
            None
        }
    }

    /// Check if coordinate is within map bounds.
    pub fn contains(&self, p: &Vec2i) -> bool {
        0 <= p.x && p.x < self.h as isize && 0 <= p.y && p.y < self.w as isize
    }

    /// Iterate over flattened map.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Get map element.
    pub fn get(&self, p: &Vec2i) -> Option<&T> {
        self.data.get(p.x as usize * self.h + p.y as usize)
    }

    /// Get mutable map element.
    pub fn get_mut(&mut self, p: &Vec2i) -> Option<&mut T> {
        self.data.get_mut(p.x as usize * self.h + p.y as usize)
    }
}

impl<T> Index<&Vec2i> for Map<T> {
    type Output = T;
    fn index(&self, p: &Vec2i) -> &Self::Output {
        &self.data[p.x as usize * self.h + p.y as usize]
    }
}

impl<T> IndexMut<&Vec2i> for Map<T> {
    fn index_mut(&mut self, p: &Vec2i) -> &mut T {
        &mut self.data[p.x as usize * self.h + p.y as usize]
    }
}
