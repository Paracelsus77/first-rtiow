use glam::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction,
            time: 0.0,
        }
    }    
    pub fn with_time(mut self, time: f32) -> Self {
        self.time = time;
        self
    }
    #[inline]
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
