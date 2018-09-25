use std::mem::replace;

use common::image::RgbaImage;

use util::sample;

pub fn filter(image: &mut RgbaImage, f_x: f32, f_y: f32) {
    let w = (image.width() as f32 * f_x) as u32;
    let h = (image.height() as f32 * f_y) as u32;

    let out = RgbaImage::from_fn(w, h, |x, y| {
        let x = x as f32 / f_x;
        let y = y as f32 / f_y;
        sample(image, x, y)
    });
    replace(image, out);
}
