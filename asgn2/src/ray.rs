use cgmath::{InnerSpace, Vector3};

/// A ray.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    /// The point at which the ray starts.
    pub origin: Vector3<f32>,

    /// A unit vector indicating the direction of the ray.
    pub direction: Vector3<f32>,
}

impl Ray {
    /// Attempts to find the collision between the row and the plane containing the given point,
    /// with the given normal.
    pub fn collide_plane(&self, point: Vector3<f32>, normal: Vector3<f32>) -> Option<f32> {
        let denom = normal.dot(self.direction);
        if denom == 0.0 {
            None
        } else {
            let numer = normal.dot(point - self.origin);
            let dist = numer / denom;
            if dist >= 0.0 {
                Some(dist)
            } else {
                None
            }
        }
    }

    /// Projects the ray out the given distance.
    pub fn project(self, distance: f32) -> Vector3<f32> {
        self.origin + distance * self.direction
    }
}
