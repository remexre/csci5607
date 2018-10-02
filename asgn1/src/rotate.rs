use common::float_ord::FloatOrd;

use util::{Image, SampleMode};

pub fn filter(image: &Image, sampler: SampleMode, theta: f32) -> Image {
    let (w, h) = image.dims();

    let (xmax, xmin, ymax, ymin) = corners(w as f32, h as f32, theta);

    Image::from_fn((xmax - xmin) as u32, (ymax - ymin) as u32, |x, y| {
        let x = x as f32 + xmin;
        let y = y as f32 + ymin;
        let (x, y) = rotate(x, y, theta);
        image.sample(sampler, x, y)
    })
}

fn corners(w: f32, h: f32, theta: f32) -> (f32, f32, f32, f32) {
    let c = [
        rotate(0.0, 0.0, -theta),
        rotate(w, 0.0, -theta),
        rotate(0.0, h, -theta),
        rotate(w, h, -theta),
    ];
    let xmax = c.iter().map(|&(x, _)| FloatOrd(x)).max().unwrap();
    let xmin = c.iter().map(|&(x, _)| FloatOrd(x)).min().unwrap();
    let ymax = c.iter().map(|&(_, y)| FloatOrd(y)).max().unwrap();
    let ymin = c.iter().map(|&(_, y)| FloatOrd(y)).min().unwrap();
    (xmax.0, xmin.0, ymax.0, ymin.0)
}

fn rotate(x: f32, y: f32, theta: f32) -> (f32, f32) {
    let x2 = y * theta.sin() + x * theta.cos();
    let y2 = y * theta.cos() - x * theta.sin();
    (x2, y2)
}
