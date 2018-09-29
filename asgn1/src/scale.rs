use util::{Image, SampleMode};

pub fn filter(image: &Image, sampler: SampleMode, f_x: f32, f_y: f32) -> Image {
    let (w, h) = image.dims();
    let w = (w as f32 * f_x) as u32;
    let h = (h as f32 * f_y) as u32;

    Image::from_fn(w, h, |x, y| {
        let x = x as f32 / f_x;
        let y = y as f32 / f_y;
        image.sample(sampler, x, y)
    })
}
