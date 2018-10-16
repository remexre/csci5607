use cgmath::{InnerSpace, Vector3};

use material::Material;
use ray::Ray;
use renderable::Renderable;

/// A sphere.
#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    /// The position of the center of the sphere.
    pub position: Vector3<f32>,

    /// The radius of the sphere.
    pub radius: f32,

    /// The material the sphere is made of.
    pub material: Material,
}

impl Renderable for Sphere {
    fn collides_with(&self, ray: Ray) -> Option<f32> {
        let a = ray.direction.magnitude2();
        let b = 2.0 * ray.direction.dot(ray.origin - self.position);
        let c = ray.origin.magnitude2() + self.position.magnitude2()
            - 2.0 * ray.origin.dot(self.position)
            - self.radius * self.radius;

        let delta = b * b - 4.0 * a * c;
        if delta >= 0.0 {
            let two_a = 2.0 * a;
            let minus_b = -b;
            let delta_sqrt = delta.sqrt();

            let t = (minus_b - delta_sqrt) / two_a;
            if t <= 0.0 {
                let t = (minus_b + delta_sqrt) / two_a;
                if t <= 0.0 {
                    None
                } else {
                    Some(t)
                }
            } else {
                Some(t)
            }
        } else {
            None
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal_at(&self, pos: Vector3<f32>) -> Vector3<f32> {
        (pos - self.position).normalize()
    }
}
