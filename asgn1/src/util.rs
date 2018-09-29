use std::ops::{AddAssign, Index, IndexMut, Mul};

use common::image::{Rgba, RgbaImage};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SampleMode {
    Bilinear,
    Gaussian,
    Point,
}

/// A 2D image, where each pixel's value is an [f32; 4].
pub struct Image {
    pixels: Vec<Pixel>,
    width: u32,
}

impl Image {
    /// Returns the dimensions of the image.
    pub fn dims(&self) -> (u32, u32) {
        let l = self.pixels.len();
        assert!(l <= ::std::u32::MAX as usize);
        let l = l as u32;

        let w = self.width;
        assert_eq!(l % w, 0);
        (w, l / w)
    }

    /// Generates an image from a function.
    pub fn from_fn<F>(w: u32, h: u32, f: F) -> Image
    where
        F: Fn(u32, u32) -> Pixel,
    {
        let mut pixels = Vec::with_capacity((w * h) as usize);
        for y in 0..h {
            for x in 0..w {
                pixels.push(f(x, y));
            }
        }
        Image { pixels, width: w }
    }

    fn idx(&self, x: u32, y: u32) -> usize {
        let i = (y as usize * self.width as usize) + x as usize;
        assert!(i < self.pixels.len());
        i
    }

    /// Samples the image at the given point.
    pub fn sample(&self, mode: SampleMode, x: f32, y: f32) -> Pixel {
        match mode {
            SampleMode::Bilinear => unimplemented!(),
            SampleMode::Gaussian => unimplemented!(),
            SampleMode::Point => {
                let x = x.round() as u32;
                let y = y.round() as u32;

                self[(x, y)]
            }
        }
    }
}

impl From<RgbaImage> for Image {
    fn from(image: RgbaImage) -> Image {
        Image::from_fn(image.width(), image.height(), |x, y| {
            let [r, g, b, a] = image[(x, y)].data;
            Pixel([
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0,
            ])
        })
    }
}

impl Index<(u32, u32)> for Image {
    type Output = Pixel;
    fn index(&self, (x, y): (u32, u32)) -> &Pixel {
        &self.pixels[self.idx(x, y)]
    }
}

impl IndexMut<(u32, u32)> for Image {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Pixel {
        let i = self.idx(x, y);
        &mut self.pixels[i]
    }
}

impl<'a> Into<RgbaImage> for &'a Image {
    fn into(self) -> RgbaImage {
        let (w, h) = self.dims();
        RgbaImage::from_fn(w, h, |x, y| {
            let Pixel([r, g, b, a]) = self[(x, y)];
            assert!(0.0 <= r && r <= 1.0);
            assert!(0.0 <= g && g <= 1.0);
            assert!(0.0 <= b && b <= 1.0);
            assert!(0.0 <= a && a <= 1.0);
            Rgba {
                data: [
                    (255.0 * r) as u8,
                    (255.0 * g) as u8,
                    (255.0 * b) as u8,
                    (255.0 * a) as u8,
                ],
            }
        })
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Pixel(pub [f32; 4]);

impl AddAssign for Pixel {
    fn add_assign(&mut self, p: Pixel) {
        self.0[0] += p.0[0];
        self.0[1] += p.0[1];
        self.0[2] += p.0[2];
        self.0[3] += p.0[3];
    }
}

impl Mul<f32> for Pixel {
    type Output = Pixel;
    fn mul(self, n: f32) -> Pixel {
        unimplemented!()
    }
}
