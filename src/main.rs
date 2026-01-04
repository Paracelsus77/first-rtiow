use glam::Vec3;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rtiow::{Camera, Hittable, HittableList, Interval, Ray, Sphere};
// use rand::Rng;

const WIDTH: usize = 1280;
const _HEIGHT: usize = 720;

fn ray_color(ray: Ray, world: &HittableList) -> Vec3 {
    if let Some(hit) = world.hit(ray, Interval::new(0.0, f32::INFINITY)) {
        0.5 * (hit.normal + 1.0)
    } else {
        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        Vec3::ONE.lerp(Vec3::new(0.5, 0.7, 1.0), a)
    }
}

fn vec3_to_u32(color: Vec3) -> u32 {
    let intensity = Interval::new(0.0, 0.999);

    let r = (intensity.clamp(color.x) * 256.0) as u32;
    let g = (intensity.clamp(color.y) * 256.0) as u32;
    let b = (intensity.clamp(color.z) * 256.0) as u32;

    (r << 16) | (g << 8) | b
}

fn rand_float() -> f32 {
    rand::random::<f32>()
}

fn random_range(min: f32, max: f32) -> f32 {
    rand::random_range(min..max) 
}

#[expect(unused)]
fn random_vec3() -> Vec3 {
    Vec3::from_array(rand::random())
}

#[expect(unused)]
fn random_in_unit_sphere() -> Vec3 {
    loop {
        // Generate a random vector between -1.0 and 1.0
        let p = Vec3::new(
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0)
        );
        let lensq = p.length_squared();
        if lensq < 1.0 && lensq > f32::EPSILON {
            return p;
        }
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(rand_float() - 0.5, rand_float() - 0.5, 0.0)
}

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

                pixel_color += ray_color(r, &world);
            }
            buffer[i + j * camera.image_width] = vec3_to_u32(pixel_color * pixel_sample_scale);
        }
    }
}

fn main() {
    let camera = Camera::new(WIDTH);

    let mut buffer = vec![0u32; camera.image_width * camera.image_height];

    let mut world = HittableList {
        objects: Vec::new(),
    };

    world.objects.push(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.objects.push(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
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
            render(&mut buffer, &camera, &world);
            redraw_needed = false;
        }

        window
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();
    }
}
