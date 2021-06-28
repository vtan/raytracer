use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy)]
pub struct V3(pub [f64; 3]);

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

impl Neg for V3 {
    type Output = V3;

    fn neg(self) -> V3 {
        let V3([x, y, z]) = self;
        V3([-x, -y, -z])
    }
}

impl Mul for V3 {
    type Output = V3;

    fn mul(self, rhs: V3) -> Self::Output {
        let V3([x1, y1, z1]) = self;
        let V3([x2, y2, z2]) = rhs;
        V3([x1 * x2, y1 * y2, z1 * z2])
    }
}

impl Mul<f64> for V3 {
    type Output = V3;

    fn mul(self, a: f64) -> V3 {
        let V3([x, y, z]) = self;
        V3([a * x, a * y, a * z])
    }
}

impl Mul<V3> for f64 {
    type Output = V3;

    fn mul(self, rhs: V3) -> V3 {
        let V3([x, y, z]) = rhs;
        V3([self * x, self * y, self * z])
    }
}

impl V3 {
    pub const ZERO: V3 = V3([0.0, 0.0, 0.0]);

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

    pub fn cross(self, rhs: V3) -> V3 {
        let V3([x1, y1, z1]) = self;
        let V3([x2, y2, z2]) = rhs;
        V3([
            y1 * z2 - z1 * y2,
            z1 * x2 - x1 * z2,
            x1 * y2 - y1 * x2
        ])
    }

    pub fn map(self, f: fn(f64) -> f64) -> V3 {
        let V3([x, y, z]) = self;
        V3([f(x), f(y), f(z)])
    }

    pub fn is_near_zero(self) -> bool {
        let eps = 1e-8;
        let V3([x, y, z]) = self;
        x.abs() < eps && y.abs() < eps && z.abs() < eps
    }

    pub fn reflect(self, normal: V3) -> V3 {
        self - normal * (self.dot(normal) * 2.0)
    }
}
