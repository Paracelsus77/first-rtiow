use glam::Vec3;

pub struct Camera {
    pub center: Vec3,
    pub pixel00_loc: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub image_width: usize, 
    pub image_height: usize, 
}

impl Camera {
    pub fn new(image_width: usize) -> Self {
        let aspect_ratio = 16.0 / 9.0f32;
        
        let image_height = ((image_width as f32 / aspect_ratio) as usize).max(1);

        let focal_length = 1f32;
        let viewport_height = 2f32;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    
        let camera_center = Vec3::ZERO;

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / (image_width as f32);
        let pixel_delta_v = viewport_v / (image_height as f32);

        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            image_width,
            image_height,
        }
    }
}
