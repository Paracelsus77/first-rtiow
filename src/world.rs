use crate::{HitRecord, Hittable, Interval, Ray, Sphere};

pub struct HittableList {
    pub objects: Vec<Sphere>,
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(hit) = object.hit(r, Interval { min: ray_t.min, max: closest_so_far }) {
                closest_so_far = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
