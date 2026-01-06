use glam::Vec3;

pub struct Camera {
    pub center: Vec3,
    pub pixel00_loc: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
    pub image_width: usize,
    pub image_height: usize,
    pub defocus_angle: f32
}

impl Camera {
    pub fn new(image_width: usize, vfov: f32, lookfrom: Vec3, lookat: Vec3, vup: Vec3, defocus_angle:f32, focus_dist: f32) -> Self {
        let aspect_ratio = 16.0 / 9.0f32;

        let image_height = ((image_width as f32 / aspect_ratio) as usize).max(1);

        // let focal_length = (lookfrom - lookat).length();

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let camera_center = lookfrom;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / (image_width as f32);
        let pixel_delta_v = viewport_v / (image_height as f32);

        let viewport_upper_left =
            camera_center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        Self {
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
            image_width,
            image_height,
            defocus_angle,
        }
    }
}
