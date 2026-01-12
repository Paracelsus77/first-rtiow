use glam::Vec3;

pub fn rand_float() -> f32 {
    rand::random::<f32>()
}

pub fn random_range(min: f32, max: f32) -> f32 {
    rand::random_range(min..max)
}

pub fn random_vec3() -> Vec3 {
    Vec3::from_array(rand::random())
}

pub fn random_vec3_range(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        // Generate a random vector between -1.0 and 1.0
        let p = Vec3::new(
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0),
        );
        let lensq = p.length_squared();
        if lensq < 1.0 && lensq > f32::EPSILON {
            return p;
        }
    }
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    let orientation = on_unit_sphere.dot(normal);
    if orientation > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn sample_square() -> Vec3 {
    Vec3::new(rand_float() - 0.5, rand_float() - 0.5, 0.0)
}

pub trait NearZero {
    fn near_zero(&self) -> bool;
}

impl NearZero for Vec3 {
    fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}
