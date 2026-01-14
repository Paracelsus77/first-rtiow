use glam::Vec3;

use crate::{HitRecord, Hittable, Interval, Material, Ray};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();

            let root = (h - sqrtd) / a;
            let t = if ray_t.surrounds(root) {
                root
            } else {
                let second_root = (h + sqrtd) / a;
                if ray_t.surrounds(second_root) {
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
                mat: self.material,
            })
        }
    }
}

pub enum Primitive{
    Sphere(Sphere),
}

impl Hittable for Primitive {
    fn hit(&self, r: Ray, t_ray: Interval) -> Option<HitRecord> {
        match self {
            Primitive::Sphere(sphere) => sphere.hit(r, t_ray),
        }
    }
}