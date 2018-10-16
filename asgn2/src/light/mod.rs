//! Lights.

mod directional;
mod point;
mod spot;

use std::fmt::Debug;

use cgmath::Vector3;
use image::Rgb;

pub use light::directional::Directional;
pub use light::point::Point;
pub use light::spot::Spot;

/// Any light defined in this crate.
///
/// Defined because it's more efficient than a trait object.
#[derive(Clone, Debug, PartialEq)]
pub enum DynamicLight {
    /// A directional light.
    Directional(Directional),

    /// A point light source.
    Point(Point),

    /// A spotlight.
    Spot(Spot),
}

impl Light for DynamicLight {
    fn color(&self) -> Rgb<f32> {
        match *self {
            DynamicLight::Directional(ref d) => d.color(),
            DynamicLight::Point(ref p) => p.color(),
            DynamicLight::Spot(ref s) => s.color(),
        }
    }

    fn direction_from(&self, point: Vector3<f32>) -> (Vector3<f32>, f32) {
        match *self {
            DynamicLight::Directional(ref d) => d.direction_from(point),
            DynamicLight::Point(ref p) => p.direction_from(point),
            DynamicLight::Spot(ref s) => s.direction_from(point),
        }
    }

    fn intensity_at(&self, point: Vector3<f32>) -> f32 {
        match *self {
            DynamicLight::Directional(ref d) => d.intensity_at(point),
            DynamicLight::Point(ref p) => p.intensity_at(point),
            DynamicLight::Spot(ref s) => s.intensity_at(point),
        }
    }
}

/// A trait for lights.
pub trait Light: Debug {
    /// Returns the color of the light.
    fn color(&self) -> Rgb<f32>;

    /// Returns a unit vector *from* the point to the light source, and a
    /// length for the distance to the light source (which may be infinite).
    ///
    /// If the length is not infinite, the following should hold:
    ///
    /// ```rust,ignore
    /// let (dir, dist) = light.direction_from(point);
    /// let ray = Ray {
    ///     origin: point,
    ///     direction: dir,
    /// };
    /// let (_, ldist) = light.direction_from(ray.project(dist));
    /// assert_eq!(ldist, 0);
    /// ```
    fn direction_from(&self, point: Vector3<f32>) -> (Vector3<f32>, f32);

    /// Returns the intensity of the light at the given point, assuming no
    /// collisions occur.
    fn intensity_at(&self, point: Vector3<f32>) -> f32;
}
