use crate::material::Material;
use crate::ray::Ray;
use crate::ray_hit::RayHit;
use crate::v3::V3;

#[derive(Clone, Copy)]
pub struct RayHitMaterial<'m> {
    pub hit: RayHit,
    pub material: &'m dyn Material,
}

pub trait Surface: Send + Sync {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<RayHitMaterial>;
}

impl<T: Surface + ?Sized> Surface for Vec<Box<T>> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<RayHitMaterial> {
        let mut nearest_result: Option<RayHitMaterial> = None;

        for surface in self.iter() {
            let nearest_t = nearest_result.map_or(t_max, |hit| hit.hit.t);
            let result = surface.hit(ray, t_min, nearest_t);
            if result.is_some() {
                nearest_result = result;
            }
        }
        nearest_result
    }
}

pub struct Sphere {
    pub center: V3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Surface for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<RayHitMaterial> {
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
                // TODO: move to RayHit?
                let mut normal = (position - self.center) * (1.0 / self.radius);
                let on_front_face = ray.direction.dot(normal) < 0.0;
                if !on_front_face {
                    normal = normal * -1.0
                }
                Some(RayHitMaterial {
                    hit: RayHit {
                        position,
                        normal,
                        t,
                        on_front_face,
                    },
                    material: &(*self.material),
                })
            }
        } else {
            None
        }
    }
}
