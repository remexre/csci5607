use cgmath::Vector3;

/// A ray.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    /// The point at which the ray starts.
    pub origin: Vector3<f32>,

    /// A unit vector indicating the direction of the ray.
    pub direction: Vector3<f32>,
}

impl Ray {
    /// Projects the ray out the given distance.
    pub fn project(self, distance: f32) -> Vector3<f32> {
        self.origin + distance * self.direction
    }
}
