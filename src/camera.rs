use crate::{ray::Ray, v3::V3};

#[derive(Clone, Copy)]
pub struct Camera {
    screen_width: f64,
    screen_height: f64,
    origin: V3,
    horizontal_extent: V3,
    vertical_extent: V3,
    lower_left_corner: V3,
}

impl Camera {
    pub fn new(screen_width: i32, screen_height: i32) -> Camera {
        let aspect_ratio = (screen_width as f64) / (screen_height as f64);
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = V3::ZERO;
        let focal_length = 1.0;
        let horizontal_extent = V3([viewport_width, 0.0, 0.0]);
        let vertical_extent = V3([0.0, viewport_height, 0.0]);
        let depth = V3([0.0, 0.0, focal_length]);
        let lower_left_corner = origin - horizontal_extent * 0.5 - vertical_extent * 0.5 - depth;
        Camera {
            screen_width: screen_width as f64,
            screen_height: screen_height as f64,
            origin,
            horizontal_extent,
            vertical_extent,
            lower_left_corner,
        }
    }

    pub fn ray_from(self, screen_x: f64, screen_y: f64) -> Ray {
        let x = screen_x / (self.screen_width - 1.0);
        let y = (self.screen_height - 1.0 - screen_y) / (self.screen_height - 1.0);
        let origin = self.origin;
        let direction =
            self.lower_left_corner + self.horizontal_extent * x + self.vertical_extent * y - origin;
        Ray { origin, direction }
    }
}
