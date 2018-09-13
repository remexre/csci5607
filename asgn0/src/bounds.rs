#[derive(Clone, Copy, Debug)]
pub enum Bounds {
    Corner,
    Edge,
    Middle,
}

impl Bounds {
    /// Gets the transformation matrix corresponding to the bounds.
    pub fn transform(self) -> [[f32; 3]; 4] {
        unimplemented!()
    }
}
