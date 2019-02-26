use cgmath::{Point3, Vector3, InnerSpace};

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

#[cfg(test)]
mod test;
