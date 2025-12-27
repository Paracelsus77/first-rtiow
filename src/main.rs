use glam::Vec3;
use minifb::{Key, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    #[expect(unused)]
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[expect(unused)]
fn ray_color(ray: Ray) -> Vec3 {
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    // (1.0 - a) * Vec3::splat(1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    Vec3::ONE.lerp(Vec3::new(0.5, 0.7, 1.0), a)
}

fn vec3_to_u32(color: Vec3) -> u32 {
    let r = (color.x * 255.999) as u32;
    let g = (color.y * 255.999) as u32;
    let b = (color.z * 255.999) as u32;

    (r << 16) | (g << 8) | b
}

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "first program rtiow",
        WIDTH,
        HEIGHT,
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
                    let color = Vec3::new(
                        i as f32 / (width - 1) as f32,
                        j as f32 / (height - 1) as f32,
                        0.0,
                    );

                    buffer[i + j * width] = vec3_to_u32(color);
                }
            }
            redraw_needed = false;
        }

        window
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();
    }
}
