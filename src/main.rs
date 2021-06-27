mod camera;
mod ray;
mod surface;
mod v3;

use camera::Camera;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};
use ray::Ray;
use surface::{Sphere, Surface};
use v3::V3;

fn main() {
    const WIDTH: usize = 1280;
    const HEIGHT: usize = 720;
    const SAMPLES_PER_PIXEL: usize = 64;
    const MAX_DEPTH: i32 = 16;

    let camera = Camera::new(WIDTH as i32, HEIGHT as i32);

    let scene: Vec<Box<dyn Surface>> = vec![
        Box::new(Sphere {
            center: V3([0.0, 0.0, -1.0]),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: V3([0.0, -100.5, -1.0]),
            radius: 100.0,
        }),
    ];

    let mut pixels = vec![v3::ZERO; WIDTH * HEIGHT];
    let mut rng = rand::thread_rng();

    for screen_x in 0..WIDTH {
        for screen_y in 0..HEIGHT {
            let mut color = v3::ZERO;
            for _ in 0..SAMPLES_PER_PIXEL {
                let x = screen_x as f64 + rng.gen::<f64>();
                let y = screen_y as f64 + rng.gen::<f64>();

                let ray = camera.ray_from(x, y);
                color = color + ray_color(ray, &scene, MAX_DEPTH);
            }
            pixels[screen_x + screen_y * WIDTH] =
                color.map(|x| (x / (SAMPLES_PER_PIXEL as f64)).sqrt())
        }
    }

    print_ppm(WIDTH, HEIGHT, &pixels);
}

type Color = V3;
const WHITE: Color = V3([1.0, 1.0, 1.0]);
const BLUE: Color = V3([0.5, 0.7, 1.0]);
const T_MIN: f64 = 0.001;

fn ray_color(ray: Ray, scene: &dyn Surface, depth: i32) -> Color {
    if depth > 0 {
        match scene.hit(ray, T_MIN, f64::INFINITY) {
            Some(hit) => {
                let child_ray = Ray {
                    origin: hit.position,
                    direction: hit.normal + random_unit_vector(),
                };
                let child_color = ray_color(child_ray, scene, depth - 1);
                child_color * 0.5
            }
            None => sky_color(ray),
        }
    } else {
        v3::ZERO
    }
}

fn sky_color(ray: Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);
    WHITE * (1.0 - t) + BLUE * t
}

fn random_unit_vector() -> V3 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let mut rng = thread_rng();
    let v = V3([
        normal.sample(&mut rng),
        normal.sample(&mut rng),
        normal.sample(&mut rng),
    ]);
    v.normalize()
}

fn print_ppm(width: usize, height: usize, pixels: &[Color]) -> () {
    fn to_byte(pixel: f64) -> u8 {
        ((256.0 * pixel) as i32).clamp(0, 255) as u8
    }
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
    for row in pixels.chunks(width) {
        for &pixel in row {
            let V3([r, g, b]) = pixel;
            print!("{} {} {} ", to_byte(r), to_byte(g), to_byte(b));
        }
        println!();
    }
}
