use crate::ray::Ray;
use crate::util::{random_unit_vector, reflect};
use crate::v3::V3;

#[derive(Clone, Copy)]
pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: V3,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: Ray, hit_position: V3, normal: V3) -> Option<ScatteredRay>;
}

#[derive(Clone, Copy)]
pub struct Diffuse {
    pub color: V3,
}

impl Material for Diffuse {
    fn scatter(&self, _ray: Ray, hit_position: V3, normal: V3) -> Option<ScatteredRay> {
        let mut direction = normal + random_unit_vector();
        if direction.is_near_zero() {
            direction = normal
        }
        Some(ScatteredRay {
            ray: Ray {
                origin: hit_position,
                direction,
            },
            attenuation: self.color,
        })
    }
}

#[derive(Clone, Copy)]
pub struct Reflective {
    pub color: V3,
    pub fuzz: f64,
}

impl Material for Reflective {
    fn scatter(&self, ray: Ray, hit_position: V3, normal: V3) -> Option<ScatteredRay> {
        let direction =
            reflect(ray.direction.normalize(), normal) + random_unit_vector() * self.fuzz;
        if direction.dot(normal) > 0.0 {
            Some(ScatteredRay {
                ray: Ray {
                    origin: hit_position,
                    direction,
                },
                attenuation: self.color,
            })
        } else {
            None
        }
    }
}
