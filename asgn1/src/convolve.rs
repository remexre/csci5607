use util::{Image, Pixel};

pub fn filter(image: &Image, matrix: [[f32; 3]; 3]) -> Image {
    let (w, h) = image.dims();
    Image::from_fn(w, h, |x, y| {
        let mut px = Pixel::default();
        for xo in 0u32..=2 {
            for yo in 0u32..=2 {
                let mut x = x + xo;
                if x == 0 {
                    x = 2;
                } else if x >= w {
                    x -= 1;
                }
                x -= 1;

                let mut y = y + yo;
                if y == 0 {
                    y = 2;
                } else if y >= h {
                    y -= 1;
                }
                y -= 1;

                if x >= w || y >= h {
                    println!("x: {} {}", x, w);
                    println!("y: {} {}", y, h);
                }
                px += image[(x, y)] * matrix[xo as usize][yo as usize];
            }
        }
        px
    })
}
