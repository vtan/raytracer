use crate::v3::V3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: V3,
    pub direction: V3,
}

impl Ray {
    pub fn at(self, t: f64) -> V3 {
        self.origin + self.direction * t
    }
}
