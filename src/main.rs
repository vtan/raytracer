mod bounding_box;
mod bounding_box_tree;
mod camera;
mod material;
mod ray;
mod ray_hit;
mod render;
mod scene;
mod surface;
mod util;
mod v3;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Mutex;

use crate::render::RenderOptions;
use crate::scene::make_scene;
use crate::v3::V3;
use png::HasParameters;
use rayon::prelude::*;

fn main() {
    const WIDTH: usize = 1280;
    const HEIGHT: usize = 720;

    let scene = make_scene();
    let bounded_scene = bounding_box_tree::build(scene.surfaces).unwrap();
    let render_options = RenderOptions {
        screen_width: WIDTH as f64,
        screen_height: HEIGHT as f64,
        aspect_ratio: WIDTH as f64 / HEIGHT as f64,
        samples_per_pixel: 16,
        max_scatter_depth: 16,
        camera: scene.camera,
        scene: bounded_scene,
    };

    let mut pixels_shared = Mutex::new(vec![V3::ZERO; WIDTH * HEIGHT]);

    (0..WIDTH * HEIGHT).into_par_iter().for_each(|i| {
        let screen_x = i % WIDTH;
        let screen_y = i / WIDTH;
        let color = render::render_pixel(&render_options, screen_x, screen_y);

        let mut pixels = pixels_shared.lock().unwrap();
        pixels[screen_x + screen_y * WIDTH] = color;
    });

    let pixels = pixels_shared.get_mut().unwrap();
    write_png(Path::new("output.png"), WIDTH, HEIGHT, pixels);
}

fn write_png(path: &Path, width: usize, height: usize, pixels: &[V3]) -> () {
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
