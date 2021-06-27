mod camera;
mod material;
mod ray;
mod surface;
mod util;
mod v3;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Mutex;

use crate::camera::Camera;
use crate::material::{Diffuse, Reflective};
use crate::ray::Ray;
use crate::surface::{Sphere, Surface};
use crate::v3::V3;
use png::HasParameters;
use rand::Rng;
use rayon::prelude::*;

fn main() {
    const WIDTH: usize = 1280;
    const HEIGHT: usize = 720;
    const SAMPLES_PER_PIXEL: usize = 64;
    const MAX_DEPTH: i32 = 32;

    let camera = Camera::new(WIDTH as i32, HEIGHT as i32);

    let red = Diffuse {
        color: V3([0.8, 0.5, 0.5]),
    };
    let gray = Diffuse {
        color: V3([0.5, 0.5, 0.5]),
    };
    let left = Reflective {
        color: V3([0.8, 0.8, 0.8]),
        fuzz: 0.05,
    };
    let right = Reflective {
        color: V3([0.8, 0.6, 0.2]),
        fuzz: 0.6,
    };

    let scene: Vec<Box<dyn Surface>> = vec![
        Box::new(Sphere {
            center: V3([0.0, 0.0, -1.0]),
            radius: 0.5,
            material: &red,
        }),
        Box::new(Sphere {
            center: V3([0.0, -100.5, -1.0]),
            radius: 100.0,
            material: &gray,
        }),
        Box::new(Sphere {
            center: V3([-1.0, 0.0, -1.0]),
            radius: 0.5,
            material: &left,
        }),
        Box::new(Sphere {
            center: V3([1.0, 0.0, -1.0]),
            radius: 0.5,
            material: &right,
        }),
    ];

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
            color = color + ray_color(ray, &scene, MAX_DEPTH);
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
            Some(hit) => match hit.material.scatter(ray, hit.position, hit.normal) {
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
