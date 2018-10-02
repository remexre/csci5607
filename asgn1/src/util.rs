use std::f32::consts::PI;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg};

use common::{
    image::{Rgba, RgbaImage},
    rayon::prelude::*,
};

/// Inter/extra-polates.
pub fn polate<T>(from: T, to: T, f: f32) -> T
where
    T: Add<Output = T>,
    T: Copy,
    T: Neg<Output = T>,
    T: Mul<f32, Output = T>,
{
    (to + -from) * f + from
}

pub fn transform_as_yuv<F>(pixel: Pixel, f: F) -> Pixel
where
    F: FnOnce(f32, f32, f32) -> (f32, f32, f32),
{
    let Pixel([r, g, b, a]) = pixel;
    let y = 0.299 * r + 0.587 * g + 0.114 * b;
    let u = 0.492 * (b - y);
    let v = 0.877 * (r - y);

    let (y, u, v) = f(y, u, v);

    let r = y + 1.14 * v;
    let g = y - 0.395 * u - 0.581 * v;
    let b = y + 2.033 * u;
    Pixel([r, g, b, a])
}

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
        F: Send + Sync,
    {
        let f = &f;
        let pixels = (0..h)
            .into_par_iter()
            .flat_map(|y| (0..w).into_par_iter().map(move |x| f(x, y)))
            .collect();
        Image { pixels, width: w }
    }

    fn idx(&self, x: u32, y: u32) -> usize {
        let i = (y as usize * self.width as usize) + x as usize;
        assert!(i < self.pixels.len());
        i
    }

    /// Normalizes every pixel.
    pub fn normalize(mut self) -> Image {
        self.pixels.par_iter_mut().for_each(|p| *p = p.normalize());
        self
    }

    /// Samples the image at the given point.
    pub fn sample(&self, mode: SampleMode, x: f32, y: f32) -> Pixel {
        let (w, h) = self.dims();
        let pt_samp = |x: f32, y: f32| {
            let x = x.round() as u32;
            let y = y.round() as u32;
            if x >= w || y >= h {
                Pixel::BLACK
            } else {
                self[(x, y)]
            }
        };

        match mode {
            SampleMode::Bilinear => {
                let ul = pt_samp(x, y);
                let ur = pt_samp(x + 1.0, y);
                let ll = pt_samp(x, y + 1.0);
                let lr = pt_samp(x + 1.0, y + 1.0);
                let x = x.fract();
                let y = y.fract();
                ul * (1.0 - x) * (1.0 - y) + ur * x * (1.0 - y) + ll * (1.0 - x) * y + lr * x * y
            }
            SampleMode::Gaussian => {
                let gauss = |x: f32| (-x.powi(2) / 2.0).exp() / (2.0 * PI).sqrt();
                let mut px = Pixel::default();
                let mut n = 0.0;
                for xo in -2..=2i32 {
                    for yo in -2..=2i32 {
                        let xo = xo as f32;
                        let yo = yo as f32;
                        let d = (xo.powi(2) + yo.powi(2)).sqrt();
                        let g = gauss(d) / 2.4610448;
                        n += g;
                        px += pt_samp(x + xo, y + yo) * g;
                    }
                }
                if n > 1.0 {
                    println!("{}", n);
                }
                px
            }
            SampleMode::Point => pt_samp(x, y),
        }
    }
}

impl Add for Image {
    type Output = Image;
    fn add(self, other: Image) -> Image {
        let (ws, hs) = self.dims();
        let (wo, ho) = other.dims();
        assert_eq!(ws, wo);
        assert_eq!(hs, ho);

        Image::from_fn(wo, ho, |x, y| self[(x, y)] + other[(x, y)])
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

impl MulAssign<f32> for Image {
    fn mul_assign(&mut self, n: f32) {
        self.pixels.par_iter_mut().for_each(|p| *p *= n);
    }
}

impl<'a> Into<RgbaImage> for &'a Image {
    fn into(self) -> RgbaImage {
        let (w, h) = self.dims();
        RgbaImage::from_fn(w, h, |x, y| {
            let Pixel([r, g, b, a]) = self[(x, y)].normalize();
            Rgba {
                data: [
                    (r * 255.0) as u8,
                    (g * 255.0) as u8,
                    (b * 255.0) as u8,
                    (a * 255.0) as u8,
                ],
            }
        })
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Pixel(pub [f32; 4]);

impl Pixel {
    /// A solid black pixel.
    pub const BLACK: Pixel = Pixel([0.0, 0.0, 0.0, 1.0]);

    /// Normalizes the pixel.
    pub fn normalize(self) -> Pixel {
        let [r, g, b, _] = self.0;
        Pixel([
            r.max(0.0).min(1.0),
            g.max(0.0).min(1.0),
            b.max(0.0).min(1.0),
            1.0,
        ])
    }
}

impl Add for Pixel {
    type Output = Pixel;
    fn add(mut self, p: Pixel) -> Pixel {
        self += p;
        self
    }
}

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
    fn mul(mut self, n: f32) -> Pixel {
        self *= n;
        self
    }
}

impl MulAssign<f32> for Pixel {
    fn mul_assign(&mut self, n: f32) {
        self.0[0] *= n;
        self.0[1] *= n;
        self.0[2] *= n;
        self.0[3] *= n;
    }
}

impl Neg for Pixel {
    type Output = Pixel;
    fn neg(mut self) -> Pixel {
        self.0[0] = -self.0[0];
        self.0[1] = -self.0[1];
        self.0[2] = -self.0[2];
        self.0[3] = -self.0[3];
        self
    }
}

impl Sum for Pixel {
    fn sum<I>(iter: I) -> Pixel
    where
        I: Iterator<Item = Pixel>,
    {
        let mut sr = 0.0;
        let mut sg = 0.0;
        let mut sb = 0.0;
        let mut sa = 0.0;
        iter.for_each(|Pixel([r, g, b, a])| {
            sr += r;
            sg += g;
            sb += b;
            sa += a;
        });
        Pixel([sr, sg, sb, sa])
    }
}
