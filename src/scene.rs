use std::f64::consts::PI;

use rand::{thread_rng, Rng};

use crate::camera::{Camera, CameraOptions};
use crate::material::{Diffuse, Material, Reflective, Refractive};
use crate::surface::{Sphere, Surface};
use crate::v3::V3;

pub struct Scene {
    pub camera: Camera,
    pub surfaces: Vec<Box<dyn Surface>>,
}

pub fn make_scene() -> Scene {
    let mut surfaces: Vec<Box<dyn Surface>> = Vec::new();
    let mut rng = thread_rng();

    surfaces.push(Box::new(Sphere {
        center: V3([0.0, -1000.0, 0.0]),
        radius: 1000.0,
        material: Box::new(Diffuse {
            color: V3([0.5, 0.5, 0.5]),
        }),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = V3([
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            ]);

            if (center - V3([4.0, 0.2, 0.0])).length() > 0.9 {
                let material: Box<dyn Material> = if choose_mat < 0.8 {
                    let c1 = V3([rng.gen(), rng.gen(), rng.gen()]);
                    let c2 = V3([rng.gen(), rng.gen(), rng.gen()]);
                    Box::new(Diffuse { color: c1 * c2 })
                } else if choose_mat < 0.95 {
                    let color = V3([
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                    ]);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Box::new(Reflective { color, fuzz })
                } else {
                    Box::new(Refractive { ratio: 1.5 })
                };
                surfaces.push(Box::new(Sphere {
                    center,
                    material,
                    radius: 0.2,
                }))
            }
        }
    }

    surfaces.push(Box::new(Sphere {
        center: V3([0.0, 1.0, 0.0]),
        radius: 1.0,
        material: Box::new(Refractive { ratio: 1.5 }),
    }));
    surfaces.push(Box::new(Sphere {
        center: V3([-4.0, 1.0, 0.0]),
        radius: 1.0,
        material: Box::new(Diffuse {
            color: V3([0.4, 0.2, 0.1]),
        }),
    }));
    surfaces.push(Box::new(Sphere {
        center: V3([4.0, 1.0, 0.0]),
        radius: 1.0,
        material: Box::new(Reflective {
            color: V3([0.7, 0.6, 0.5]),
            fuzz: 0.0,
        }),
    }));

    let camera = Camera::new(CameraOptions {
        look_from: V3([13.0, 2.0, 3.0]),
        look_at: V3([0.0, 0.0, 0.0]),
        vertical_field_of_view: PI / 6.0,
        aperture: 0.2,
        focus_distance: Some(10.0),
    });

    Scene { camera, surfaces }
}
