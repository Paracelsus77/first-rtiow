use glam::Vec3;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rtiow::{Hittable, HittableList, Interval, Ray, Sphere, Camera};

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
    let r = (color.x * 255.999) as u32;
    let g = (color.y * 255.999) as u32;
    let b = (color.z * 255.999) as u32;

    (r << 16) | (g << 8) | b
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

        let (width, height) = new_size;

        if redraw_needed {
            for j in 0..height {
                for i in 0..width {
                    let pixel_center =
                        camera.pixel00_loc + (i as f32 * camera.pixel_delta_u) + (j as f32 * camera.pixel_delta_v);
                    let ray_direction = pixel_center - camera.center;
                    let r = Ray {
                        origin: camera.center,
                        direction: ray_direction,
                    };

                    let pixel_color = ray_color(r, &world);
                    buffer[i + j * width] = vec3_to_u32(pixel_color);
                }
            }
            redraw_needed = false;
        }

        window
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();
    }
}
