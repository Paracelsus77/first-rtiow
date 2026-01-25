use glam::Vec3;

use crate::{Aabb, Interval, Material, Ray};

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat: Material,
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_ray: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}
