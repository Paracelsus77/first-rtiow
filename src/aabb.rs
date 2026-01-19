use glam::{Vec3, Vec3A};

use crate::{Interval, Ray};

pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Vec3, b: Vec3) -> Self {
        let min = a.min(b);
        let max = a.max(b);

        Self {
            x: Interval::new(min.x, max.x),
            y: Interval::new(min.y, max.y),
            z: Interval::new(min.z, max.z),
        }
    }

    #[inline]
    pub fn min_point(&self) -> Vec3 {
        Vec3::new(self.x.min, self.y.min, self.z.min)
    }

    #[inline]
    pub fn max_point(&self) -> Vec3 {
        Vec3::new(self.x.max, self.y.max, self.z.max)
    }

    pub fn union(box0: Self, box1: Self) -> Self {
        Aabb::from_points(
            box0.min_point().min(box1.min_point()),
            box0.max_point().max(box1.max_point()),
        )
    }

    pub fn hit(&self, r: Ray, ray_t: Interval) -> bool {
        // https://tavianator.com/2011/ray_box.html 
        // Fast, Branchless Ray/Bounding Box Intersections implementation suggested by Gemini
        // actual implementation model available from Ray Tracing Gems II 
        // chapter 2 Ray Axis-Aligned Bounding Box Intersection

        let ray_origin = Vec3A::from(r.origin);
        let ray_inv_dir = Vec3A::from(r.inv_direction);

        let box_min = Vec3A::new(self.x.min, self.y.min, self.z.min);
        let box_max = Vec3A::new(self.x.max, self.y.max, self.z.max);

        let t1 = (box_min - ray_origin) * ray_inv_dir;
        let t2 = (box_max - ray_origin) * ray_inv_dir;

        let t_min_vec = t1.min(t2);
        let t_max_vec = t1.max(t2);
        
        let t_enter = t_min_vec.max_element().max(ray_t.min);
        let t_exit = t_max_vec.min_element().min(ray_t.min);

        t_enter < t_exit
    }
}
