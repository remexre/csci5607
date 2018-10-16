use cgmath::{InnerSpace, Vector3};
use image::Rgb;

use light::Light;

/// A spotlight.
#[derive(Clone, Debug, PartialEq)]
pub struct Spot {
    /// The color of the light.
    pub color: Rgb<f32>,

    /// The position the light is emitted from.
    pub position: Vector3<f32>,

    /// The direction the light is pointing in.
    pub direction: Vector3<f32>,

    /// The intensity of the light.
    pub intensity: f32,

    /// The angle at which the light starts falling off.
    pub falloff_angle: f32,

    /// The angle after which this light ceases to have an effect.
    pub max_angle: f32,
}

impl Light for Spot {
    fn color(&self) -> Rgb<f32> {
        self.color
    }

    fn direction_from(&self, point: Vector3<f32>) -> (Vector3<f32>, f32) {
        // TODO: Should this check that the point is inside the cone of
        // max_angle?
        let v = self.position - point;
        (v.normalize(), v.magnitude())
    }

    fn intensity_at(&self, _point: Vector3<f32>) -> f32 {
        unimplemented!()
    }
}
