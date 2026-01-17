use std::time::Instant;

use glam::Vec3;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use rtiow::{
    Camera, Dielectric, Hittable, HittableList, Interval, Lambertian, Material, Metal, Primitive,
    Ray, Sphere, rand_float, random_in_unit_disk, random_range, random_vec3, random_vec3_range,
    sample_square,
};

const WIDTH: usize = 800;
const _HEIGHT: usize = 720;
const MAX_DEPTH: u32 = 8;
const SAMPLES_PER_PIXEL: u32 = 32;

fn ray_color(ray: Ray, world: &HittableList, depth: u32) -> Vec3 {
    if depth <= 0 {
        Vec3::ZERO
    } else if let Some(hit) = world.hit(ray, Interval::new(0.001, f32::INFINITY)) {
        if let Some((attenuation, direction)) = hit.mat.scatter(ray, &hit) {
            attenuation * ray_color(direction, &world, depth - 1)
        } else {
            Vec3::ZERO
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        Vec3::ONE.lerp(Vec3::new(0.5, 0.7, 1.0), a)
    }
}

fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

fn vec3_to_u32(color: Vec3) -> u32 {
    let colorx = linear_to_gamma(color.x);
    let colory = linear_to_gamma(color.y);
    let colorz = linear_to_gamma(color.z);

    let r = (colorx.clamp(0.0, 0.999) * 256.0) as u32;
    let g = (colory.clamp(0.0, 0.999) * 256.0) as u32;
    let b = (colorz.clamp(0.0, 0.999) * 256.0) as u32;

    (r << 16) | (g << 8) | b
}

fn defocus_disk_sample(center: Vec3, defocus_disk_u: Vec3, defocus_disk_v: Vec3) -> Vec3 {
    let p = random_in_unit_disk();
    center + (p.x * defocus_disk_u) + (p.y * defocus_disk_v)
}

fn render_parallel(buffer: &mut [u32], camera: &Camera, world: &HittableList) {
    let samples_per_pixel = SAMPLES_PER_PIXEL;
    let pixel_sample_scale = 1.0 / samples_per_pixel as f32;

    buffer
        .par_chunks_mut(camera.image_width)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, pixel) in row.iter_mut().enumerate() {
                let mut pixel_color = Vec3::ZERO;
                for _ in 0..samples_per_pixel {
                    let offset = sample_square();
                    let pixel_center = camera.pixel00_loc
                        + ((x as f32 + offset.x) * camera.pixel_delta_u)
                        + ((y as f32 + offset.y) * camera.pixel_delta_v);
                    let ray_origin = if camera.defocus_angle <= 0.0 {
                        camera.center
                    } else {
                        defocus_disk_sample(
                            camera.center,
                            camera.defocus_disk_u,
                            camera.defocus_disk_v,
                        )
                    };
                    let ray_direction = pixel_center - ray_origin;
                    let ray_time = rand_float();
                    let r = Ray::new(ray_origin, ray_direction).with_time(ray_time);

                    pixel_color += ray_color(r, &world, MAX_DEPTH);
                }
                *pixel = vec3_to_u32(pixel_color * pixel_sample_scale);
            }
        });
}

fn main() {
    let camera = Camera::new(
        WIDTH,
        20.0,
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );

    let mut buffer = vec![0u32; camera.image_width * camera.image_height];

    let mut world = HittableList {
        objects: Vec::new(),
    };

    // Ground
    world.objects.push(Primitive::Sphere(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rand_float();
            let sphere_center =
                Vec3::new(a as f32 + 0.9 * rand_float(), 0.2, b as f32 + rand_float());

            if (sphere_center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                world.objects.push(Primitive::Sphere(Sphere {
                    center: sphere_center,
                    radius: 0.2,
                    material: match choose_material {
                        0.0..0.8 => Material::Lambertian(Lambertian {
                            albedo: random_vec3() * random_vec3(),
                        }),
                        0.8..0.95 => Material::Metal(Metal {
                            albedo: random_vec3_range(0.5, 1.0),
                            fuzz: random_range(0.0, 0.5),
                        }),
                        _ => Material::Dielectric(Dielectric {
                            refraction_index: 1.5,
                        }),
                    },
                }))
            }
        }
    }

    world.objects.push(Primitive::Sphere(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric(Dielectric {
            refraction_index: 1.5,
        }),
    }));

    world.objects.push(Primitive::Sphere(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian(Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        }),
    }));

    world.objects.push(Primitive::Sphere(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    let mut window = Window::new(
        "first program rtiow",
        camera.image_width,
        camera.image_height,
        WindowOptions {
            resize: false,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    window.set_target_fps(100);

    let mut redraw_needed = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_size = window.get_size();

        if redraw_needed {
            let start = Instant::now();
            render_parallel(&mut buffer, &camera, &world);
            let duration = start.elapsed();
            println!("Render time: {:.2?}", duration);
            redraw_needed = false;
        }

        window
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();
    }
}
