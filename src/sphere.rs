use glam::Vec3;

use crate::{HitRecord, Hittable, Ray};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();

            let valid_root = |t| t > t_min && t < t_max;

            let root = (h - sqrtd) / a;
            let t = if valid_root(root) {
                root
            } else {
                let second_root = (h + sqrtd) / a;
                if valid_root(second_root) {
                    second_root
                } else {
                    return None;
                }
            };

            let p = r.at(t);
            let outward_normal = (p - self.center) / self.radius;
            let front_face = r.direction.dot(outward_normal) < 0.0;
            let normal = if front_face {
                outward_normal
            } else {
                -outward_normal
            };

            Some(HitRecord {
                t,
                p,
                normal,
                front_face,
            })
        }
    }
}
