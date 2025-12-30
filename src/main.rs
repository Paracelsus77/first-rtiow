use glam::Vec3;
use minifb::{Key, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 1280;
const _HEIGHT: usize = 720;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();

            let valid_root = |t| t > t_min && t < t_max;

            let root = (h - sqrtd) / a;
            let t = if valid_root(root) {
                root
            } else {
                let second_root = (h + sqrtd) / a;
                if valid_root(second_root) {
                    second_root
                } else {
                    return None;
                }
            };

            let p = r.at(t);
            let outward_normal = (p - self.center) / self.radius;
            let front_face = r.direction.dot(outward_normal) < 0.0;
            let normal = if front_face { outward_normal } else { -outward_normal };

            Some(HitRecord { t, p, normal, front_face })
        }
    }
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = center - ray.origin;
    let a = ray.direction.length_squared();
    let h = ray.direction.dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        0.5 * (n + 1.0)
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

                    let pixel_color = ray_color(r);
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
