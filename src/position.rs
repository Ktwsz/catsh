use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone)]
pub struct Position {
    pub x: f64,
    y: f64,
}

impl Position {
    pub fn x_i32(&self) -> i32 {
        self.x as i32
    }

    pub fn y_i32(&self) -> i32 {
        self.y as i32
    }


    pub fn new(x: f64, y: f64) -> Self {
        Position {x, y}
    }


    pub fn zero() -> Self {
        Position {
            x: 0.0,
            y: 0.0,
        }
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }

}

impl Mul<f64> for Position {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
