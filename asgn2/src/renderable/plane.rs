use cgmath::{InnerSpace, Vector3};

use material::Material;
use ray::Ray;
use renderable::Renderable;

/// An infinite plane.
#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
    /// An arbitrary point on the plane.
    pub point: Vector3<f32>,

    /// The normal vector from the plane.
    pub normal: Vector3<f32>,

    /// The material the plane is made of.
    pub material: Material,
}

impl Renderable for Plane {
    fn collides_with(&self, ray: Ray) -> Option<f32> {
        let denom = self.normal.dot(ray.direction);
        if denom == 0.0 {
            None
        } else {
            let numer = self.normal.dot(self.point - ray.origin);
            let dist = numer / denom;
            if dist >= 0.0 {
                Some(dist)
            } else {
                None
            }
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal_at(&self, _pos: Vector3<f32>) -> Vector3<f32> {
        // TODO: Should this fail is pos isn't on the plane?
        self.normal
    }
}
