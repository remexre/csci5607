use cgmath::Vector3;

use material::Material;
use ray::Ray;
use renderable::Renderable;

/// A triangle.
#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    /// The vertices of the triangle.
    pub vertices: (Vector3<f32>, Vector3<f32>, Vector3<f32>),

    /// The normal vector from the triangle.
    pub normal: Vector3<f32>,

    /// The material the triangle is made of.
    pub material: Material,
}

impl Renderable for Triangle {
    fn collides_with(&self, _ray: Ray) -> Option<f32> {
        unimplemented!()
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal_at(&self, _pos: Vector3<f32>) -> Vector3<f32> {
        unimplemented!()
    }
}
