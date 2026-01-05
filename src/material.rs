use crate::{HitRecord, NearZero, Ray, random_unit_vector};
use glam::Vec3;

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let scatter_direction = rec.normal + random_unit_vector();

        Some((
            self.albedo,
            Ray {
                origin: rec.p,
                direction: if scatter_direction.near_zero() {
                    rec.normal
                } else {
                    scatter_direction
                },
            },
        ))
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = r_in.direction.reflect(rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected,
        };
        Some((self.albedo, scattered))
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub ir: f32,
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match self {
            Material::Lambertian(m) => m.scatter(r_in, rec),
            Material::Metal(m) => m.scatter(r_in, rec),
            _ => None,
        }
    }
}
