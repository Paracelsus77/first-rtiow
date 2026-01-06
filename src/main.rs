use std::f32::consts::PI;

use glam::Vec3;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use rtiow::{
    Camera, Dielectric, Hittable, HittableList, Interval, Lambertian, Material, Metal, Ray, Sphere,
    sample_square,
};

const WIDTH: usize = 1280;
const _HEIGHT: usize = 720;
const MAX_DEPTH: u32 = 50;

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
    let intensity = Interval::new(0.0, 0.999);

    let colorx = linear_to_gamma(color.x);
    let colory = linear_to_gamma(color.y);
    let colorz = linear_to_gamma(color.z);

    let r = (intensity.clamp(colorx) * 256.0) as u32;
    let g = (intensity.clamp(colory) * 256.0) as u32;
    let b = (intensity.clamp(colorz) * 256.0) as u32;

    (r << 16) | (g << 8) | b
}

#[expect(unused)]
fn render(buffer: &mut [u32], camera: &Camera, world: &HittableList) {
    let samples_per_pixel = 100;
    let pixel_sample_scale = 1.0 / samples_per_pixel as f32;

    for j in 0..camera.image_height {
        for i in 0..camera.image_width {
            let mut pixel_color = Vec3::ZERO;

            for _ in 0..samples_per_pixel {
                let offset = sample_square();
                let pixel_center = camera.pixel00_loc
                    + ((i as f32 + offset.x) * camera.pixel_delta_u)
                    + ((j as f32 + offset.y) * camera.pixel_delta_v);
                let ray_direction = pixel_center - camera.center;
                let r = Ray {
                    origin: camera.center,
                    direction: ray_direction,
                };

                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            buffer[i + j * camera.image_width] = vec3_to_u32(pixel_color * pixel_sample_scale);
        }
    }
}

fn render_parallel(buffer: &mut [u32], camera: &Camera, world: &HittableList) {
    let samples_per_pixel = 100;
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
                    let ray_direction = pixel_center - camera.center;
                    let r = Ray {
                        origin: camera.center,
                        direction: ray_direction,
                    };
                    pixel_color += ray_color(r, &world, MAX_DEPTH);
                    *pixel = vec3_to_u32(pixel_color * pixel_sample_scale);
                }
            }
        });
}

fn main() {
    let camera = Camera::new(
        WIDTH,
        90.0,
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut buffer = vec![0u32; camera.image_width * camera.image_height];

    let mut world = HittableList {
        objects: Vec::new(),
    };

    // let r = (PI / 4.0).sin();

    // world.objects.push(Sphere {
    //     center: Vec3::new(-r, 0.0, -1.0),
    //     radius: r,
    //     material: Material::Lambertian(Lambertian {
    //         albedo: Vec3::new(0.0, 0.0, 1.0),
    //     }),
    // });

    // world.objects.push(Sphere {
    //     center: Vec3::new(r, 0.0, -1.0),
    //     radius: r,
    //     material: Material::Lambertian(Lambertian {
    //         albedo: Vec3::new(1.0, 0.0, 0.0),
    //     }),
    // });

    world.objects.push(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Material::Lambertian(Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0),
        }),
    });

    world.objects.push(Sphere {
        center: Vec3::new(0.0, 0.0, -1.2),
        radius: 0.5,
        material: Material::Lambertian(Lambertian {
            albedo: Vec3::new(0.1, 0.2, 0.5),
        }),
    });

    world.objects.push(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Dielectric(Dielectric {
            refraction_index: 1.5,
        }),
    });

    world.objects.push(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.4,
        material: Material::Dielectric(Dielectric {
            refraction_index: 1.0 / 1.5,
        }),
    });

    world.objects.push(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Metal(Metal {
            albedo: Vec3::new(0.8, 0.6, 0.2),
            fuzz: 1.0,
        }),
    });

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
            render_parallel(&mut buffer, &camera, &world);
            redraw_needed = false;
        }

        window
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();
    }
}
