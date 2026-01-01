use glam::Vec3;

use crate::{Interval, Ray};

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_ray: Interval) -> Option<HitRecord>;
}
