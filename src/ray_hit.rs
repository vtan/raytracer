use crate::v3::V3;

#[derive(Clone, Copy)]
pub struct RayHit {
    pub position: V3,
    pub normal: V3,
    pub t: f64,
    pub on_front_face: bool,
}
