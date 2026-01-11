use crate::{HitRecord, NearZero, Ray, rand_float, random_unit_vector};
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
                time: _r_in.time,
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
            direction: reflected.normalize() + (self.fuzz * random_unit_vector()),
            time: r_in.time,
        };
        if scattered.direction.dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub refraction_index: f32,
}

fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.normalize();

        let cos_theta = (-unit_direction.dot(rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > rand_float() {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, ri)
        };

        // let refracted = r_in.direction.normalize().refract(rec.normal, ri);
        Some((
            Vec3::ONE,
            Ray {
                origin: rec.p,
                direction: direction,
                time: r_in.time,
            },
        ))
    }
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
            Material::Dielectric(m) => m.scatter(r_in, rec),
        }
    }
}
