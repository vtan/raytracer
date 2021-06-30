use rand::Rng;

use crate::camera::Camera;
use crate::ray::Ray;
use crate::surface::Surface;
use crate::v3::V3;

type Color = V3;
const SKY_TOP: Color = V3([1.0, 1.0, 1.0]);
const SKY_BOTTOM: Color = V3([0.5, 0.7, 1.0]);
const T_MIN: f64 = 0.00001;

pub struct RenderOptions {
    pub screen_width: f64,
    pub screen_height: f64,
    pub aspect_ratio: f64,
    pub samples_per_pixel: i32,
    pub max_scatter_depth: i32,
    pub camera: Camera,
    pub scene: Box<dyn Surface>,
}

pub fn render_pixel(opts: &RenderOptions, pixel_x: usize, pixel_y: usize) -> Color {
    let mut rng = rand::thread_rng();
    let mut color = V3::ZERO;
    for _ in 0..opts.samples_per_pixel {
        let sample_x = pixel_x as f64 + rng.gen::<f64>();
        let sample_y = pixel_y as f64 + rng.gen::<f64>();

        let normalized_x = (2.0 * sample_x / opts.screen_width - 1.0) * opts.aspect_ratio;
        let normalized_y = -(2.0 * sample_y / opts.screen_height - 1.0);

        let ray = opts.camera.ray_from(normalized_x, normalized_y);
        color = color + ray_color(ray, &(*opts.scene), opts.max_scatter_depth);
    };
    color.map(|x| (x / (opts.samples_per_pixel as f64)).sqrt())
}

fn ray_color(ray: Ray, scene: &dyn Surface, depth: i32) -> Color {
    if depth > 0 {
        match scene.hit(ray, T_MIN, f64::INFINITY) {
            Some(result) => match result.material.scatter(ray, result.hit) {
                Some(scattered_ray) => {
                    scattered_ray.attenuation * ray_color(scattered_ray.ray, scene, depth - 1)
                }
                None => V3::ZERO,
            },
            None => sky_color(ray),
        }
    } else {
        V3::ZERO
    }
}

fn sky_color(ray: Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);
    SKY_TOP * (1.0 - t) + SKY_BOTTOM * t
}
