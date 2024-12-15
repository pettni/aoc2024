use crate::vec2::{Dir, Vec2i};
use std::{
    fmt,
    ops::{Index, IndexMut},
    str::Lines,
};

/// 2D map type.
///
/// Map<T> is indexed by Vec2i using "image" coordinates, i.e.
/// X-east, Y-south.
#[derive(Debug, Clone)]
pub struct Map<T> {
    pub h: usize,
    pub w: usize,
    data: Vec<T>, // row-major
}

impl<T> Map<T> {
    /// Create map filled with constant.
    pub fn new(h: usize, w: usize) -> Map<T>
    where
        T: Clone + Default,
    {
        Map {
            h,
            w,
            data: vec![T::default(); h * w],
        }
    }

    /// Create map filled with constant.
    pub fn new_constant(h: usize, w: usize, t: T) -> Map<T>
    where
        T: Clone,
    {
        Map {
            h,
            w,
            data: vec![t; h * w],
        }
    }

    /// Create map from 2d iterator.
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

    /// Create map from lines.
    pub fn from_lines<F>(lines: Lines, f: &F) -> Map<T>
    where
        F: Fn(char) -> T,
    {
        let iters = lines.map(|l| l.chars().map(f));
        Map::from_iterators(iters)
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
        let new = pos.step(dir, d as i64);
        if self.contains(&new) {
            Some(new)
        } else {
            None
        }
    }

    /// Check if coordinate is within map bounds.
    pub fn contains(&self, p: &Vec2i) -> bool {
        p.is_in_grid(self.h, self.w)
    }

    /// Iterate over coordinates.
    pub fn iter_coords(&self) -> impl Iterator<Item = Vec2i> {
        let h = self.h;
        let w = self.w;
        (0..h).flat_map(move |r| {
            (0..w).map(move |c| Vec2i {
                x: c as i64,
                y: r as i64,
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

impl<T> fmt::Display for Map<T>
where
    T: fmt::Display + Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.h {
            for col in 0..self.w {
                write!(f, "{:}", self.data[self.w * row + col])?
            }
            if row < self.h - 1 {
                writeln!(f)?
            }
        }
        Ok(())
    }
}

/// Index with Vec2i = (x, y).
impl<T> Index<&Vec2i> for Map<T> {
    type Output = T;
    fn index(&self, p: &Vec2i) -> &Self::Output {
        &self.data[p.linear_idx(self.w)]
    }
}

/// Mutable index with Vec2i = (x, y).
impl<T> IndexMut<&Vec2i> for Map<T> {
    fn index_mut(&mut self, p: &Vec2i) -> &mut Self::Output {
        &mut self.data[p.linear_idx(self.w)]
    }
}

/// Index with two integers (y, x).
impl<T> Index<(usize, usize)> for Map<T> {
    type Output = T;
    fn index(&self, yx: (usize, usize)) -> &Self::Output {
        let (y, x) = yx;
        &self.data[y * self.w + x]
    }
}

/// Mutable index with two integers (y, x).
impl<T> IndexMut<(usize, usize)> for Map<T> {
    fn index_mut(&mut self, yx: (usize, usize)) -> &mut Self::Output {
        let (y, x) = yx;
        &mut self.data[y * self.w + x]
    }
}

/// Row indexing.
impl<T> Index<usize> for Map<T> {
    type Output = [T];
    fn index(&self, y: usize) -> &Self::Output {
        &self.data[y * self.w..(y + 1) * self.w]
    }
}

/// Mutable row indexing.
impl<T> IndexMut<usize> for Map<T> {
    fn index_mut(&mut self, y: usize) -> &mut Self::Output {
        &mut self.data[y * self.w..(y + 1) * self.w]
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
        assert_eq!(map[&p], 2);
        assert_eq!(map[(0, 1)], 2);

        let p = Vec2i { x: 0, y: 1 };
        assert_eq!(map[&p], 5);
        assert_eq!(map[(1, 0)], 5);
    }

    #[test]
    fn test_map_index() {
        let data = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let mut map = Map::from_vecs(data.clone());
        for y in 0..3 {
            for x in 0..4 {
                assert_eq!(map[(y, x)], data[y][x]);
                assert_eq!(
                    map[&Vec2i {
                        x: x as i64,
                        y: y as i64
                    }],
                    data[y][x]
                );
            }
            assert_eq!(map[y], data[y]);
        }

        // mutable indexing
        map[1][1] = 66;
        map[(1, 2)] = 77;
        map[&Vec2i { x: 3, y: 1 }] = 88;
        assert_eq!(map[&Vec2i { x: 1, y: 1 }], 66);
        assert_eq!(map[&Vec2i { x: 2, y: 1 }], 77);
        assert_eq!(map[&Vec2i { x: 3, y: 1 }], 88);
    }

    #[test]
    fn test_map_access() {
        let data = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let map = Map::from_vecs(data);

        let p = Vec2i { x: 2, y: 1 };
        assert_eq!(*map.get(&p).unwrap(), 7);

        let p = Vec2i { x: 0, y: -1 };
        assert_eq!(map.get(&p), None);

        let p = Vec2i { x: -1, y: 2 };
        assert_eq!(map.get(&p), None);

        let p = Vec2i { x: 1, y: 10 };
        assert_eq!(map.get(&p), None);
    }

    #[test]
    fn test_map_iter() {
        let data = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let map = Map::from_vecs(data);

        let mut iter = map.iter();

        for i in 0..12 {
            let item = iter.next().unwrap();
            assert_eq!(item.0, Vec2i { y: i / 4, x: i % 4 });
            assert_eq!(*item.1, i + 1);
        }
    }
}
