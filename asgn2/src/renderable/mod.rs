//! Renderable objects.

mod plane;
mod sphere;
mod triangle;

use std::fmt::Debug;

use cgmath::Vector3;

use material::Material;
use ray::Ray;
pub use renderable::plane::Plane;
pub use renderable::sphere::Sphere;
pub use renderable::triangle::Triangle;

/// Any renderable defined in this crate.
///
/// Defined because it's more efficient than a trait object.
#[derive(Clone, Debug, PartialEq)]
pub enum DynamicRenderable {
    /// A plane.
    Plane(Plane),

    /// A sphere.
    Sphere(Sphere),

    /// A triangle.
    Triangle(Triangle),
}

impl Renderable for DynamicRenderable {
    fn collides_with(&self, ray: Ray) -> Option<f32> {
        match *self {
            DynamicRenderable::Plane(ref p) => p.collides_with(ray),
            DynamicRenderable::Sphere(ref s) => s.collides_with(ray),
            DynamicRenderable::Triangle(ref t) => t.collides_with(ray),
        }
    }

    fn material(&self) -> Material {
        match *self {
            DynamicRenderable::Plane(ref p) => p.material(),
            DynamicRenderable::Sphere(ref s) => s.material(),
            DynamicRenderable::Triangle(ref t) => t.material(),
        }
    }

    fn normal_at(&self, pos: Vector3<f32>) -> Vector3<f32> {
        match *self {
            DynamicRenderable::Plane(ref p) => p.normal_at(pos),
            DynamicRenderable::Sphere(ref s) => s.normal_at(pos),
            DynamicRenderable::Triangle(ref t) => t.normal_at(pos),
        }
    }
}

/// A trait for renderable objects.
pub trait Renderable: Debug {
    /// Returns whether a ray collides with this Renderable. If so, it returns
    /// the distance from the ray's origin the collision occurred at and the
    /// normal vector at that position. If not, returns `None`.
    fn collides_with(&self, ray: Ray) -> Option<f32>;

    /// Returns the material the object is made of.
    fn material(&self) -> Material;

    /// Returns the normal vector to the Renderable at the given point. If the
    /// given point does not intersect the Renderable, results may be
    /// unpredictable.
    fn normal_at(&self, pos: Vector3<f32>) -> Vector3<f32>;
}
