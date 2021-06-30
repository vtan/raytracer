use std::mem::swap;

use crate::ray::Ray;
use crate::v3::V3;

#[derive(Clone, Copy)]
pub struct BoundingBox {
    pub minimum: V3,
    pub maximum: V3,
}

impl BoundingBox {
    pub fn is_hit_by(self, ray: Ray, t_min: f64, t_max: f64) -> bool {
        for i in 0..3 {
            let direction_inv = 1.0 / ray.direction.0[i];
            let mut t0 = direction_inv * (self.minimum.0[i] - ray.origin.0[i]);
            let mut t1 = direction_inv * (self.maximum.0[i] - ray.origin.0[i]);
            if direction_inv < 0.0 {
                swap(&mut t0, &mut t1)
            }
            if t0 < t_min {
                t0 = t_min
            }
            if t1 > t_max {
                t1 = t_max
            }
            if t0 >= t1 {
                return false;
            }
        }
        true
    }

    pub fn union(self, rhs: BoundingBox) -> BoundingBox {
        let V3([x_min1, y_min1, z_min1]) = self.minimum;
        let V3([x_max1, y_max1, z_max1]) = self.maximum;
        let V3([x_min2, y_min2, z_min2]) = rhs.minimum;
        let V3([x_max2, y_max2, z_max2]) = rhs.maximum;
        BoundingBox {
            minimum: V3([
                f64::min(x_min1, x_min2),
                f64::min(y_min1, y_min2),
                f64::min(z_min1, z_min2),
            ]),
            maximum: V3([
                f64::max(x_max1, x_max2),
                f64::max(y_max1, y_max2),
                f64::max(z_max1, z_max2),
            ]),
        }
    }
}
