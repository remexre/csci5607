use util::{Image, Pixel};

/// TODO: What to do for edges? rn, trims a pixel from each edge...
pub fn filter(image: &Image, matrix: [[f32; 3]; 3]) -> Image {
    let (w, h) = image.dims();
    Image::from_fn(w - 2, h - 2, |x, y| {
        let mut px = Pixel::default();
        for xo in 0u32..=2 {
            for yo in 0u32..=2 {
                px += image[(x + xo, y + yo)] * matrix[xo as usize][yo as usize];
            }
        }
        px
    })
}
