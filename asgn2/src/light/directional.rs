use std::f32;

use cgmath::Vector3;
use image::Rgb;

use light::Light;

/// A directional light.
#[derive(Clone, Debug, PartialEq)]
pub struct Directional {
    /// The color of the light.
    pub color: Rgb<f32>,

    /// The direction the light is pointing in.
    pub direction: Vector3<f32>,

    /// The intensity of the light.
    pub intensity: f32,
}

impl Light for Directional {
    fn color(&self) -> Rgb<f32> {
        self.color
    }

    fn direction_from(&self, _point: Vector3<f32>) -> (Vector3<f32>, f32) {
        (self.direction, f32::INFINITY)
    }

    fn intensity_at(&self, _point: Vector3<f32>) -> f32 {
        self.intensity
    }
}
