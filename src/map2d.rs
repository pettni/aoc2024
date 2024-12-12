use crate::vec2::{Dir, Vec2i};
use std::ops::{Index, IndexMut};

/// 2D map type.
#[derive(Debug, Clone)]
pub struct Map<T> {
    pub h: usize,
    pub w: usize,
    data: Vec<T>, // row-major
}

impl<T> Map<T> {
    /// Create map from 2d matrix.
    pub fn from_iterators<I, J>(iter: I) -> Map<T>
    where
        I: Iterator<Item = J>,
        J: Iterator<Item = T>,
    {
        let mut h = 0;
        let mut w = 0;
        let mut data = Vec::new();

        for (i, row) in iter.enumerate() {
            h = h.max(i + 1);
            for (j, x) in row.enumerate() {
                w = w.max(j + 1);
                data.push(x);
            }
        }
        Map { h, w, data }
    }

    /// Create map from 2d matrix.
    pub fn from_vecs(vecs: Vec<Vec<T>>) -> Map<T> {
        let h = vecs.len();
        let w = vecs[0].len();
        let data: Vec<_> = vecs.into_iter().flatten().collect();
        assert_eq!(data.len(), h * w);
        Map { h, w, data }
    }

    /// Move in direction within map.
    pub fn step_within(&self, pos: &Vec2i, dir: Dir, d: isize) -> Option<Vec2i> {
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

    /// Iterate over coordinates.
    pub fn iter_coords(&self) -> impl Iterator<Item = Vec2i> {
        let h = self.h;
        let w = self.w;
        (0..h).flat_map(move |r| {
            (0..w).map(move |c| Vec2i {
                x: r as isize,
                y: c as isize,
            })
        })
    }

    /// Iterate over (coord, val) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (Vec2i, &T)> {
        self.iter_coords().map(|c| (c, &self[&c]))
    }

    /// Iterate over flattened map.
    pub fn iter_values(&self) -> impl Iterator<Item = &T> {
        self.iter_coords().map(|c| &self[&c])
    }

    /// Get map element.
    pub fn get(&self, p: &Vec2i) -> Option<&T> {
        match self.contains(p) {
            true => self.data.get(p.linear_idx(self.w)),
            false => None,
        }
    }

    /// Get mutable map element.
    pub fn get_mut(&mut self, p: &Vec2i) -> Option<&mut T> {
        match self.contains(p) {
            true => self.data.get_mut(p.linear_idx(self.w)),
            false => None,
        }
    }
}

impl<T> Index<&Vec2i> for Map<T> {
    type Output = T;
    fn index(&self, p: &Vec2i) -> &Self::Output {
        &self.data[p.linear_idx(self.w)]
    }
}

impl<T> IndexMut<&Vec2i> for Map<T> {
    fn index_mut(&mut self, p: &Vec2i) -> &mut T {
        &mut self.data[p.linear_idx(self.w)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_construct() {
        let data = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let map = Map::from_vecs(data);
        assert_eq!(map.h, 3);
        assert_eq!(map.w, 4);

        let p = Vec2i { x: 1, y: 0 };
        assert_eq!(map[&p], 5);
    }

    #[test]
    fn test_map_access() {
        let data = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let map = Map::from_vecs(data);

        let p = Vec2i { x: 1, y: 2 };
        assert_eq!(*map.get(&p).unwrap(), 7);

        let p = Vec2i { x: -1, y: 0 };
        assert_eq!(map.get(&p), None);

        let p = Vec2i { x: 2, y: -1 };
        assert_eq!(map.get(&p), None);

        let p = Vec2i { x: 10, y: 1 };
        assert_eq!(map.get(&p), None);
    }

    #[test]
    fn test_map_iter() {
        let data = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let map = Map::from_vecs(data);

        let mut iter = map.iter();

        for i in 0..12 {
            let item = iter.next().unwrap();
            assert_eq!(item.0, Vec2i { x: i / 4, y: i % 4 });
            assert_eq!(*item.1, i + 1);
        }
    }
}
