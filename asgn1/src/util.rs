use common::image::{Rgba, RgbaImage};

/// Samples an image at the given point.
pub fn sample(image: &RgbaImage, x: f32, y: f32) -> Rgba<u8> {
    //let x = (x + if x % 1.0 < 0.5 { 0.0 } else { 1.0 }) as u32;
    //let y = (y + if y % 1.0 < 0.5 { 0.0 } else { 1.0 }) as u32;
    let x = x as u32;
    let y = y as u32;

    if x < image.width() && y < image.height() {
        image[(x, y)]
    } else {
        warn!(
            "Invalid access ({}, {}) ({}, {})",
            x,
            y,
            image.width(),
            image.height()
        );
        Rgba { data: [0, 0, 0, 0] }
    }
}
