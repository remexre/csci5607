use cgmath::Vector3;

use ray::Ray;
use scene::Scene;

impl Scene {
    pub(crate) fn make_camera_ray(&self, x: u32, y: u32) -> Ray {
        let x = self.x_to_ray(x);
        let y = self.y_to_ray(y);
        Ray {
            origin: self.camera_position,
            direction: self.camera_direction + x * self.camera_right() + y * self.camera_up,
        }
    }

    /// Returns the vector pointing out from the right of the camera.
    pub fn camera_right(&self) -> Vector3<f32> {
        self.camera_up.cross(self.camera_direction)
    }

    fn aspect_ratio(&self) -> f32 {
        (self.width as f32) / (self.height as f32)
    }

    fn x_to_ray(&self, x: u32) -> f32 {
        let x = (2.0 * x as f32 / self.width as f32) - 1.0;
        return x * self.camera_half_angle_tan * self.aspect_ratio();
    }

    fn y_to_ray(&self, y: u32) -> f32 {
        let y = 1.0 - (2.0 * y as f32 / self.height as f32);
        y * self.camera_half_angle_tan
    }
}
