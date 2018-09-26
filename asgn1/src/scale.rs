use common::image::RgbaImage;

use util::SampleMode;

pub fn filter(image: &RgbaImage, sampler: SampleMode, f_x: f32, f_y: f32) -> RgbaImage {
    let w = (image.width() as f32 * f_x) as u32;
    let h = (image.height() as f32 * f_y) as u32;

    RgbaImage::from_fn(w, h, |x, y| {
        let x = x as f32 / f_x;
        let y = y as f32 / f_y;
        sampler.sample(image, x, y)
    })
}
