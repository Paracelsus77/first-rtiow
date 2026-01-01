use glam::Vec3;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rtiow::{Ray, Hittable, HittableList, Sphere};

const WIDTH: usize = 1280;
const _HEIGHT: usize = 720;

fn ray_color(ray: Ray, world: &HittableList) -> Vec3 {
    if let Some(hit) = world.hit(&ray, 0.0, f32::INFINITY) {
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
    let aspect_ratio = 16.0 / 9.0f32;
    let image_width: usize = WIDTH;

    let image_height = ((image_width as f32 / aspect_ratio) as usize).max(1);

    let focal_length = 1f32;
    let viewport_height = 2f32;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Vec3::ZERO;

    println!(
        "aspect_ratio: {}, image_width: {}, image_height: {}, focal_length: {}, viewport_height: {}, viewport_width: {}",
        aspect_ratio, image_width, image_height, focal_length, viewport_height, viewport_width
    );

    println!("camera_center: {}", camera_center);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / (image_height as f32);

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut buffer = vec![0u32; image_width * image_height];

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
        image_width,
        image_height,
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
                        pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
                    let ray_direction = pixel_center - camera_center;
                    let r = Ray {
                        origin: camera_center,
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
