use crate::bounding_box::BoundingBox;
use crate::ray::Ray;
use crate::surface::{RayHitMaterial, Surface};

struct BoundingBoxLeaf {
    surface: Box<dyn Surface>,
    bounding_box: BoundingBox,
}

impl Surface for BoundingBoxLeaf {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<RayHitMaterial> {
        if self.bounding_box.is_hit_by(ray, t_min, t_max) {
            self.surface.hit(ray, t_min, t_max)
        } else {
            None
        }
    }

    fn calculate_bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

struct BoundingBoxTree {
    left: Box<dyn Surface>,
    right: Box<dyn Surface>,
    bounding_box: BoundingBox,
}

impl Surface for BoundingBoxTree {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<RayHitMaterial> {
        if self.bounding_box.is_hit_by(ray, t_min, t_max) {
            let left_hit = self.left.hit(ray, t_min, t_max);
            let right_hit = self
                .right
                .hit(ray, t_min, left_hit.map_or(t_max, |hit| hit.hit.t));
            right_hit.or(left_hit)
        } else {
            None
        }
    }

    fn calculate_bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

pub fn build(surfaces: Vec<Box<dyn Surface>>) -> Option<Box<dyn Surface>> {
    let surfaces_with_bounding_box = surfaces
        .into_iter()
        .map(|surface| {
            let bounding_box = surface.calculate_bounding_box();
            Box::new(BoundingBoxLeaf {
                surface,
                bounding_box,
            })
        })
        .collect();
    let depth = 0;
    build_rec(surfaces_with_bounding_box, depth)
}

fn build_rec(mut leaves: Vec<Box<BoundingBoxLeaf>>, depth: usize) -> Option<Box<dyn Surface>> {
    match leaves.len() {
        0 => None,
        1 => leaves.into_iter().next().map(|x| x as Box<dyn Surface>),
        length => {
            if depth >= 6 {
                let bounding_box = leaves.calculate_bounding_box();
                let surface = Box::new(leaves);
                Some(Box::new(BoundingBoxLeaf {
                    surface,
                    bounding_box,
                }))
            } else {
                let axis = depth % 3;
                let sort_key = |x: &BoundingBoxLeaf| x.bounding_box.minimum.0[axis];
                leaves.sort_unstable_by(|a, b| sort_key(a).partial_cmp(&sort_key(b)).unwrap());
                let right_children = leaves.split_off(length / 2);
                let left_children = leaves;
                let left = build_rec(left_children, depth + 1)?;
                let right = build_rec(right_children, depth + 1)?;
                let bounding_box = left
                    .calculate_bounding_box()
                    .union(right.calculate_bounding_box());
                Some(Box::new(BoundingBoxTree {
                    left,
                    right,
                    bounding_box,
                }))
            }
        }
    }
}
