use cgmath::{Point3, Vector3, InnerSpace};
use std::cmp::{PartialOrd, Ord, Ordering};

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    ray_t: f64,
    origin: Point3<f64>,
    normal: Vector3<f64>,
}

impl Intersection {
    pub fn new(ray_t: f64, origin: Point3<f64>, normal: Vector3<f64>) -> Self {
        let normal = normal.normalize();

        Self { ray_t, origin, normal }
    }
}

impl Eq for Intersection { }

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ray_t.partial_cmp(&other.ray_t)
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("invalid value for ray_t")
    }
}

#[cfg(test)]
mod test;
