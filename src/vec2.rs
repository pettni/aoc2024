use std::ops::{Add, DivAssign, Mul, Neg, Sub};

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

impl Neg for Dir {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.turn_around()
    }
}

// 2D coordinate type
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vec2i {
    pub x: isize,
    pub y: isize,
}

impl Vec2i {
    #[inline]
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Move in direction.
    pub fn step(&self, dir: Dir, d: isize) -> Self {
        let (dx, dy) = match dir {
            Dir::N => (-d, 0),
            Dir::E => (0, d),
            Dir::S => (d, 0),
            Dir::W => (0, -d),
        };
        Self::new(self.x + dx, self.y + dy)
    }

    /// Contained in [0, h)x(0, w)
    pub fn is_in_grid(&self, h: usize, w: usize) -> bool {
        self.x >= 0 && self.x < h as isize && self.y >= 0 && self.y < w as isize
    }

    /// Get linear row-major index.
    pub fn linear_idx(&self, w: usize) -> usize {
        (self.x * w as isize + self.y) as usize
    }
}

impl Add for Vec2i {
    type Output = Self;
    fn add(self, rhs: Vec2i) -> Self::Output {
        Vec2i::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2i {
    type Output = Self;
    fn sub(self, rhs: Vec2i) -> Self::Output {
        Vec2i::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<isize> for Vec2i {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Vec2i::new(self.x * rhs, self.y * rhs)
    }
}

impl DivAssign<isize> for Vec2i {
    fn div_assign(&mut self, rhs: isize) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
