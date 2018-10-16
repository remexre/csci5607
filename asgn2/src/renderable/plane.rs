use cgmath::Vector3;

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
        ray.collide_plane(self.point, self.normal)
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal_at(&self, _pos: Vector3<f32>) -> Vector3<f32> {
        // TODO: Should this fail if pos isn't on the plane?
        self.normal
    }
}
