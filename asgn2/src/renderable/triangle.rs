use cgmath::{dot, Vector3};

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

impl Triangle {
    /// Calculates barycentric coordinates on the triangle.
    fn barycentric(&self, p: Vector3<f32>) -> (f32, f32) {
        // Current implementation stolen from https://gamedev.stackexchange.com/a/49370
        let v0 = self.vertices.1 - self.vertices.0;
        let v1 = self.vertices.2 - self.vertices.0;
        let v2 = p - self.vertices.0;
        let d00 = dot(v0, v0);
        let d01 = dot(v0, v1);
        let d11 = dot(v1, v1);
        let d20 = dot(v2, v0);
        let d21 = dot(v2, v1);
        let inv_denom = 1.0 / (d00 * d11 - d01 * d01);
        let alpha = (d11 * d20 - d01 * d21) * inv_denom;
        let beta = (d00 * d21 - d01 * d20) * inv_denom;
        (alpha, beta)
    }
}

impl Renderable for Triangle {
    fn collides_with(&self, ray: Ray) -> Option<f32> {
        let dist = ray.collide_plane(self.vertices.0, self.normal)?;
        let p = ray.project(dist);
        let (alpha, beta) = self.barycentric(p);
        if alpha >= 0.0 && beta >= 0.0 && alpha + beta <= 1.0 {
            Some(dist)
        } else {
            None
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal_at(&self, _pos: Vector3<f32>) -> Vector3<f32> {
        // TODO: Should this fail if pos isn't on the plane of the triangle?
        self.normal
    }
}
