use common::image::{Rgba, RgbaImage};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SampleMode {
    Bilinear,
    Gaussian,
    Point,
}

impl SampleMode {
    /// Samples an image at the given point.
    pub fn sample(self, image: &RgbaImage, x: f32, y: f32) -> Rgba<u8> {
        match self {
            SampleMode::Bilinear => unimplemented!(),
            SampleMode::Gaussian => unimplemented!(),
            SampleMode::Point => {
                let x = x.round() as u32;
                let y = y.round() as u32;

                image[(x, y)]
            }
        }
    }
}
