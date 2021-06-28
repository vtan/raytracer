use crate::v3::V3;
use rand::thread_rng;
use rand_distr::{Distribution, UnitSphere};

pub fn random_unit_vector() -> V3 {
    V3(UnitSphere.sample(&mut thread_rng()))
}
