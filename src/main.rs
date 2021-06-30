mod bounding_box;
mod bounding_box_tree;
mod camera;
mod material;
mod ray;
mod ray_hit;
mod scene;
mod surface;
mod util;
mod v3;

use std::f64::consts::PI;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Mutex;

use crate::camera::{Camera, CameraOptions};
use crate::ray::Ray;
use crate::scene::make_scene;
use crate::surface::Surface;
use crate::v3::V3;
use png::HasParameters;
use rand::Rng;
use rayon::prelude::*;

fn main() {
    const WIDTH: usize = 1280;
    const HEIGHT: usize = 720;
    const SAMPLES_PER_PIXEL: usize = 32;
    const MAX_DEPTH: i32 = 32;

    let camera = Camera::new(CameraOptions {
        screen_width: WIDTH as i32,
        screen_height: HEIGHT as i32,
        look_from: V3([13.0, 2.0, 3.0]),
        look_at: V3([0.0, 0.0, 0.0]),
        vertical_field_of_view: PI / 6.0,
        aperture: 0.1,
        focus_distance: Some(10.0),
    });
    let scene = bounding_box_tree::build(make_scene()).unwrap();

    let mut pixels_shared = Mutex::new(vec![V3::ZERO; WIDTH * HEIGHT]);

    (0..WIDTH * HEIGHT).into_par_iter().for_each(|i| {
        let screen_x = i % WIDTH;
        let screen_y = i / WIDTH;

        let mut rng = rand::thread_rng();
        let mut color = V3::ZERO;
        for _ in 0..SAMPLES_PER_PIXEL {
            let x = screen_x as f64 + rng.gen::<f64>();
            let y = screen_y as f64 + rng.gen::<f64>();
            let ray = camera.ray_from(x, y);
            color = color + ray_color(ray, &(*scene), MAX_DEPTH);
        }

        let mut pixels = pixels_shared.lock().unwrap();
        pixels[screen_x + screen_y * WIDTH] = color.map(|x| (x / (SAMPLES_PER_PIXEL as f64)).sqrt())
    });

    let pixels = pixels_shared.get_mut().unwrap();
    write_png(Path::new("output.png"), WIDTH, HEIGHT, pixels);
}

type Color = V3;
const WHITE: Color = V3([1.0, 1.0, 1.0]);
const BLUE: Color = V3([0.5, 0.7, 1.0]);
const T_MIN: f64 = 0.00001;

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
    WHITE * (1.0 - t) + BLUE * t
}

fn write_png(path: &Path, width: usize, height: usize, pixels: &[Color]) -> () {
    let bytes: Vec<u8> = pixels
        .iter()
        .flat_map(|color| {
            let V3(components) = color;
            components
                .iter()
                .map(|&pixel| ((256.0 * pixel) as i32).clamp(0, 255) as u8)
        })
        .collect();
    let file = File::create(path).unwrap();
    let writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, width as u32, height as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    encoder
        .write_header()
        .unwrap()
        .write_image_data(&bytes)
        .unwrap();
}
