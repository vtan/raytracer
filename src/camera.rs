use rand::thread_rng;
use rand_distr::{Distribution, UnitDisc};

use crate::{ray::Ray, v3::V3};

#[derive(Clone, Copy)]
pub struct Camera {
    screen_width: f64,
    screen_height: f64,
    origin: V3,
    horizontal_extent: V3,
    vertical_extent: V3,
    lower_left_corner: V3,
    u: V3,
    v: V3,
    lens_radius: f64,
}

#[derive(Clone, Copy)]
pub struct CameraOptions {
    pub screen_width: i32,
    pub screen_height: i32,
    pub look_from: V3,
    pub look_at: V3,
    pub vertical_field_of_view: f64,
    pub aperture: f64,
    pub focus_distance: Option<f64>,
}

impl Camera {
    pub fn new(opts: CameraOptions) -> Camera {
        let aspect_ratio = (opts.screen_width as f64) / (opts.screen_height as f64);
        let viewport_height = 2.0 * (opts.vertical_field_of_view / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let w = (opts.look_from - opts.look_at).normalize();
        let u = V3([0.0, 1.0, 0.0]).cross(w).normalize();
        let v = w.cross(u);

        let origin = opts.look_from;
        let focus_distance = opts.focus_distance.unwrap_or((opts.look_from - opts.look_at).length());
        let horizontal_extent = focus_distance * viewport_width * u;
        let vertical_extent = focus_distance * viewport_height * v;
        let lower_left_corner =
            origin - 0.5 * horizontal_extent - 0.5 * vertical_extent - focus_distance * w;
        Camera {
            screen_width: opts.screen_width as f64,
            screen_height: opts.screen_height as f64,
            origin,
            horizontal_extent,
            vertical_extent,
            lower_left_corner,
            u,
            v,
            lens_radius: opts.aperture / 2.0,
        }
    }

    pub fn ray_from(self, screen_x: f64, screen_y: f64) -> Ray {
        let [random_x, random_y]: [f64; 2] = UnitDisc.sample(&mut thread_rng());
        let offset = self.lens_radius * (random_x * self.u + random_y * self.v);

        let x = screen_x / (self.screen_width - 1.0);
        let y = (self.screen_height - 1.0 - screen_y) / (self.screen_height - 1.0);
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner
                + x * self.horizontal_extent
                + y * self.vertical_extent
                - self.origin
                - offset,
        }
    }
}
