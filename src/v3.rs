use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct V3(pub [f64; 3]);

pub const ZERO: V3 = V3([0.0, 0.0, 0.0]);

impl Add for V3 {
    type Output = V3;

    fn add(self, other: V3) -> V3 {
        let V3([x1, y1, z1]) = self;
        let V3([x2, y2, z2]) = other;
        V3([x1 + x2, y1 + y2, z1 + z2])
    }
}

impl Sub for V3 {
    type Output = V3;

    fn sub(self, other: V3) -> V3 {
        let V3([x1, y1, z1]) = self;
        let V3([x2, y2, z2]) = other;
        V3([x1 - x2, y1 - y2, z1 - z2])
    }
}

impl Mul<f64> for V3 {
    type Output = V3;

    fn mul(self, a: f64) -> V3 {
        let V3([x, y, z]) = self;
        V3([a * x, a * y, a * z])
    }
}

impl V3 {
    pub fn y(self) -> f64 {
        self.0[1]
    }

    pub fn dot(self, other: V3) -> f64 {
        let V3([x1, y1, z1]) = self;
        let V3([x2, y2, z2]) = other;
        x1 * x2 + y1 * y2 + z1 * z2
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> V3 {
        self * (1.0 / self.length())
    }
}
