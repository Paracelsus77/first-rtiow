use minifb::{Key, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "first program rtiow",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    window.set_target_fps(100);

    let mut size = (0, 0);

    let mut redraw_needed = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_size = window.get_size();
        if new_size != size {
            size = new_size;
            buffer.resize(size.0 * size.1, 0);
            redraw_needed = true;
        }
        let (width, height) = new_size;

        if redraw_needed {
            for j in 0..height {
                for i in 0..width {
                    let r = i as f64 / (width - 1) as f64;
                    let g = j as f64 / (height - 1) as f64;
                    let b = 0.0;

                    let ir = (r * 255.999) as u32;
                    let ig = (g * 255.999) as u32;
                    let ib = (b * 255.999) as u32;

                    buffer[i + j * width] = (ir << 16) | (ig << 8) | ib
                }
            }
            redraw_needed = false;
        }

        window
            .update_with_buffer(&buffer, new_size.0, new_size.1)
            .unwrap();

    }
}
