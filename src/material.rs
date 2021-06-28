use rand::{thread_rng, Rng};

use crate::ray::Ray;
use crate::ray_hit::RayHit;
use crate::util::random_unit_vector;
use crate::v3::V3;

#[derive(Clone, Copy)]
pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: V3,
}

pub trait Material: Send + Sync {
    fn scatter(&self, ray: Ray, hit: RayHit) -> Option<ScatteredRay>;
}

#[derive(Clone, Copy)]
pub struct Diffuse {
    pub color: V3,
}

impl Material for Diffuse {
    fn scatter(&self, _ray: Ray, hit: RayHit) -> Option<ScatteredRay> {
        let mut direction = hit.normal + random_unit_vector();
        if direction.is_near_zero() {
            direction = hit.normal
        }
        Some(ScatteredRay {
            ray: Ray {
                origin: hit.position,
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
    fn scatter(&self, ray: Ray, hit: RayHit) -> Option<ScatteredRay> {
        let direction =
            ray.direction.normalize().reflect(hit.normal) + random_unit_vector() * self.fuzz;
        if direction.dot(hit.normal) > 0.0 {
            Some(ScatteredRay {
                ray: Ray {
                    origin: hit.position,
                    direction,
                },
                attenuation: self.color,
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Refractive {
    pub ratio: f64,
}

impl Material for Refractive {
    fn scatter(&self, ray: Ray, hit: RayHit) -> Option<ScatteredRay> {
        let adjusted_ratio = if hit.on_front_face {
            1.0 / self.ratio
        } else {
            self.ratio
        };
        let unit_direction = ray.direction.normalize();

        let cos_angle = hit.normal.dot(-unit_direction).min(1.0);
        let is_refracting = {
            let sin_angle = (1.0 - cos_angle * cos_angle).sqrt();
            adjusted_ratio * sin_angle <= 1.0
                || Self::reflectance(cos_angle, adjusted_ratio) > thread_rng().gen()
        };

        let direction = if is_refracting {
            let perpendicular = (unit_direction + hit.normal * cos_angle) * adjusted_ratio;
            let parallel = hit.normal * -(1.0 - perpendicular.length_squared()).abs().sqrt();
            perpendicular + parallel
        } else {
            unit_direction.reflect(hit.normal)
        };
        Some(ScatteredRay {
            ray: Ray {
                origin: hit.position,
                direction,
            },
            attenuation: V3([1.0, 1.0, 1.0]),
        })
    }
}

impl Refractive {
    fn reflectance(cos_angle: f64, ratio: f64) -> f64 {
        let x = 1.0 - cos_angle;
        let mut y = x;
        y = y * y;
        y = y * y * x;
        let mut r = (1.0 - ratio) / (1.0 + ratio);
        r = r * r;
        r + (1.0 - r) * y
    }
}
