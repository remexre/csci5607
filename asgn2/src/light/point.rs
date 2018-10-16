use cgmath::{InnerSpace, Vector3};
use image::Rgb;

use light::Light;

/// A point light source.
#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    /// The color of the light.
    pub color: Rgb<f32>,

    /// The position the light is emitted from.
    pub position: Vector3<f32>,

    /// The intensity of the light.
    pub intensity: f32,
}

impl Light for Point {
    fn color(&self) -> Rgb<f32> {
        self.color
    }

    fn direction_from(&self, point: Vector3<f32>) -> (Vector3<f32>, f32) {
        let v = self.position - point;
        (v.normalize(), v.magnitude())
    }

    fn intensity_at(&self, point: Vector3<f32>) -> f32 {
        10.0 * self.intensity * (point - self.position).magnitude2().recip()
    }
}
