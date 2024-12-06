use std::ops::{Index, IndexMut};

// 2D direction type
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dir {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
}

impl Dir {
    pub fn turn_right(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    pub fn turn_left(&self) -> Dir {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        }
    }

    pub fn turn_around(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
        }
    }
}

// 2D coordinate type
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos {
    pub r: usize,
    pub c: usize,
}

impl Pos {
    /// Move in direction.
    pub fn step(&self, dir: &Dir, d: isize) -> Option<Pos> {
        let (dr, dc) = match dir {
            Dir::N => (-d, 0),
            Dir::E => (0, d),
            Dir::S => (d, 0),
            Dir::W => (0, -d),
        };
        let new = Pos {
            r: self.r.checked_add_signed(dr)?,
            c: self.c.checked_add_signed(dc)?,
        };
        Some(new)
    }
}

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
    pub fn step_within(&self, pos: &Pos, dir: &Dir, d: isize) -> Option<Pos> {
        let new = pos.step(dir, d)?;
        if self.contains(&new) {
            Some(new)
        } else {
            None
        }
    }

    /// Check if coordinate is within map bounds.
    pub fn contains(&self, pos: &Pos) -> bool {
        pos.r < self.h && pos.c < self.w
    }

    /// Iterate over flattened map.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Get map element.
    pub fn get(&self, p: &Pos) -> Option<&T> {
        self.data.get(p.r * self.h + p.c)
    }

    /// Get mutable map element.
    pub fn get_mut(&mut self, p: &Pos) -> Option<&mut T> {
        self.data.get_mut(p.r * self.h + p.c)
    }
}

impl<T> Index<&Pos> for Map<T> {
    type Output = T;
    fn index(&self, p: &Pos) -> &Self::Output {
        &self.data[p.r * self.h + p.c]
    }
}

impl<T> IndexMut<&Pos> for Map<T> {
    fn index_mut(&mut self, p: &Pos) -> &mut T {
        &mut self.data[p.r * self.h + p.c]
    }
}
