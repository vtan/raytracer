use crate::ray::Ray;
use crate::v3::V3;

#[derive(Clone, Copy)]
pub struct RayHit {
    pub position: V3,
    pub normal: V3,
    pub t: f64,
}

pub trait Surface {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<RayHit>;
}

impl Surface for Vec<Box<dyn Surface>> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<RayHit> {
        let mut nearest_hit: Option<RayHit> = None;

        for surface in self.iter() {
            let nearest_t = nearest_hit.map_or(t_max, |hit| hit.t);
            let hit = surface.hit(ray, t_min, nearest_t);
            if hit.is_some() {
                nearest_hit = hit;
            }
        }
        nearest_hit
    }
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: V3,
    pub radius: f64,
}

impl Surface for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<RayHit> {
        let center = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b_half = center.dot(ray.direction);
        let c = center.length_squared() - self.radius * self.radius;

        let discriminant = b_half * b_half - a * c;
        if discriminant >= 0.0 {
            let discriminant_sqrt = discriminant.sqrt();
            let mut root = (-b_half - discriminant_sqrt) / a;
            if root < t_min || root > t_max {
                root = (-b_half + discriminant_sqrt) / a;
            }
            if root < t_min || root > t_max {
                None
            } else {
                let t = root;
                let position = ray.at(t);
                let normal = (position - self.center) * (1.0 / self.radius);
                Some(RayHit { position, normal, t })
            }
        } else {
            None
        }
    }
}
