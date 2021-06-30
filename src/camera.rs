use rand::thread_rng;
use rand_distr::{Distribution, UnitDisc};

use crate::ray::Ray;
use crate::v3::V3;

#[derive(Clone, Copy)]
pub struct Camera {
    origin: V3,
    x_unit: V3,
    y_unit: V3,
    z_unit: V3,
    viewport_height: f64,
    focus_distance: f64,
    lens_radius: f64,
}

#[derive(Clone, Copy)]
pub struct CameraOptions {
    pub look_from: V3,
    pub look_at: V3,
    pub vertical_field_of_view: f64,
    pub aperture: f64,
    pub focus_distance: Option<f64>,
}

impl Camera {
    pub fn new(opts: CameraOptions) -> Camera {
        let viewport_height = (opts.vertical_field_of_view / 2.0).tan();

        let z_unit = (opts.look_from - opts.look_at).normalize();
        let x_unit = V3([0.0, 1.0, 0.0]).cross(z_unit).normalize();
        let y_unit = z_unit.cross(x_unit);

        let focus_distance = opts
            .focus_distance
            .unwrap_or((opts.look_from - opts.look_at).length());
        Camera {
            origin: opts.look_from,
            x_unit,
            y_unit,
            z_unit,
            viewport_height,
            focus_distance,
            lens_radius: opts.aperture / 2.0,
        }
    }

    pub fn ray_from(self, normalized_x: f64, normalized_y: f64) -> Ray {
        let [random_x, random_y]: [f64; 2] = UnitDisc.sample(&mut thread_rng());
        let offset = self.lens_radius * (random_x * self.x_unit + random_y * self.y_unit);
        Ray {
            origin: self.origin + offset,
            direction: self.focus_distance
                * (self.viewport_height
                    * (normalized_x * self.x_unit + normalized_y * self.y_unit)
                    - self.z_unit)
                - offset,
        }
    }
}
