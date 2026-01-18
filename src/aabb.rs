use glam::Vec3;

use crate::Interval;

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
}
