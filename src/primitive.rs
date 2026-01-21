use glam::Vec3;

use crate::{Aabb, HitRecord, Hittable, Interval, Material, Ray};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

pub struct MovingSphere {
    pub center: Vec3,
    pub offset: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,

        }
    }

    pub fn moving(self, offset: Vec3) -> MovingSphere {
        MovingSphere {
            center: self.center,
            offset,
            radius: self.radius,
            material: self.material,
        }
    }
}

impl MovingSphere {
    #[inline]
    pub fn at(&self, t: f32) -> Vec3 {
        self.center + t * (self.offset)
    }
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
    
    fn bounding_box(&self) -> Aabb {
        let rvec = Vec3::splat(self.radius);

        Aabb::from_points(self.center-rvec, self.center+rvec)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.at(r.time);
        let oc = current_center - r.origin;
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
            let outward_normal = (p - current_center) / self.radius;
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
    
    fn bounding_box(&self) -> Aabb {
        let rvec = Vec3::splat(self.radius);
        let center2 = self.center + self.offset; 
        let bbox2 = Aabb::from_points(center2-rvec, center2+rvec);
        Aabb::union( Aabb::from_points(self.center-rvec, self.center+rvec), bbox2)
        
    }
}

pub enum Primitive {
    Sphere(Sphere),
    MovingSphere(MovingSphere),
}

impl Hittable for Primitive {
    #[inline]
    fn hit(&self, r: Ray, t_ray: Interval) -> Option<HitRecord> {
        match self {
            Primitive::Sphere(sphere) => sphere.hit(r, t_ray),
            Primitive::MovingSphere(sphere) => sphere.hit(r, t_ray),
        }
    }
    
    fn bounding_box(&self) -> Aabb {
        todo!()
    }
}
