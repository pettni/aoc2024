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
    pub x: i64,
    pub y: i64,
}

impl Vec2i {
    #[inline]
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// Move in direction.
    pub fn step(&self, dir: Dir, d: i64) -> Self {
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
        self.x >= 0 && self.x < h as i64 && self.y >= 0 && self.y < w as i64
    }

    /// Get linear row-major index.
    pub fn linear_idx(&self, w: usize) -> usize {
        (self.x * w as i64 + self.y) as usize
    }
}

impl Add<Vec2i> for Vec2i {
    type Output = Self;
    fn add(self, rhs: Vec2i) -> Self::Output {
        Vec2i::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<i64> for Vec2i {
    type Output = Self;
    fn add(self, rhs: i64) -> Self::Output {
        Vec2i::new(self.x + rhs, self.y + rhs)
    }
}

impl Sub for Vec2i {
    type Output = Self;
    fn sub(self, rhs: Vec2i) -> Self::Output {
        Vec2i::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i64> for Vec2i {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Vec2i::new(self.x * rhs, self.y * rhs)
    }
}

impl DivAssign<i64> for Vec2i {
    fn div_assign(&mut self, rhs: i64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
