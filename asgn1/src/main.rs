extern crate common;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

mod args;
mod blur;
mod convolve;
mod pipe;
mod rotate;
mod scale;
mod util;

use std::env::args;
use std::process::exit;
use std::time::Instant;

use common::{
    image::{open as open_image, RgbaImage},
    rand::random,
    rayon::prelude::*,
    run_err,
};

use args::Filter;

use util::{polate, transform_as_yuv, Image, Pixel, SampleMode};

fn main() {
    ::common::stderrlog::new().verbosity(3).init().ok();
    let args = match args::parse(args()) {
        Some(args) => args,
        None => {
            eprintln!(
                "USAGE: {} -input PATH [FILTERS...]",
                args().next().unwrap_or_else(|| "asgn1".to_string())
            );
            const USAGE: &str = r#"
FILTERS:
    -blur                           Performs a Gaussian blur.
    -brighten F                     Brightens by the given factor.
    -channel CHANNEL                Extracts a single channel (red, green, blue).
    -contrast F                     Adjusts the contrast by the given factor.
    -crop T L H W                   Crops the image.
    -edge-detect                    Detects edges.
    -edge-detect-base               Runs the edge-detection filter at the core of the -edge-detect
                                    filter.
    -floyd-steinberg-dither BITS    Performs Floyd-Steinberg dithering.
    -grayscale                      Transforms the image to grayscale.
    -output PATH                    Writes the current state of the image to the given path.
    -pipe COMMAND                   Pipes the image as a JPEG to the given command.
    -quantitize BITS                Quantitizes the image to have the given number of bits.
    -random-dither BITS             Quantitizes the image to have the given number of bits, with
                                    random dithering.
    -random-noise                   Adds random noise.
    -rotate DEGS                    Rotates the image by the given angle.
    -sample bilinear                Sets sampling mode to bilinear.
    -sample gaussian                Sets sampling mode to Gaussian.
    -sample point                   Sets sampling mode to point.
    -saturation F                   Adjusts the saturation by the given factor.
    -scale X Y                      Scales the image by the given factor in the X and Y directions.
    -sharpen                        Sharpens the image.

NOTES:
    Filters are applied one after the other, left to right. The upper-left corner is the origin,
    with the X axis going right and the Y axis going down."#;
            eprintln!("{}", USAGE);
            exit(-1);
        }
    };

    run_err(move || {
        let mut image: Image = open_image(args.input)?.to_rgba().into();
        let mut sample_mode = SampleMode::Point;

        for filter in args.filters {
            debug!("Applying {:?}...", filter);
            let start = Instant::now();
            match filter {
                Filter::BlackWhiteLuminosity(cutoff) => {
                    let (w, h) = image.dims();
                    image = Image::from_fn(w, h, |x, y| {
                        let Pixel([r, g, b, a]) = image[(x, y)];
                        let l = 0.3 * r + 0.6 * g + 0.1 * b;
                        let n = if l > cutoff { 1.0 } else { 0.0 };
                        Pixel([n, n, n, a])
                    })
                }
                Filter::Blur => {
                    image = blur::filter(&image);
                }
                Filter::Brighten(f) => {
                    let (w, h) = image.dims();
                    image = Image::from_fn(w, h, |x, y| {
                        let Pixel([r, g, b, a]) = image[(x, y)];
                        Pixel([r * f, g * f, b * f, a])
                    })
                }
                Filter::Crop(top, left, width, height) => {
                    image = Image::from_fn(width, height, |x, y| {
                        image.sample(sample_mode, x as f32 + left, y as f32 + top)
                    });
                }
                Filter::Channel(ch) => {
                    let (w, h) = image.dims();
                    image = Image::from_fn(w, h, |x, y| {
                        let Pixel([r, g, b, a]) = image[(x, y)];
                        let mut m = [0.0; 3];
                        m[ch] = 1.0;
                        Pixel([r * m[0], g * m[1], b * m[2], a])
                    })
                }
                Filter::Contrast(f) => {
                    let (w, h) = image.dims();
                    let sum: Pixel = (0..h)
                        .into_par_iter()
                        .flat_map(|y| {
                            let image = &image;
                            (0..w).into_par_iter().map(move |x| image[(x, y)])
                        }).sum();
                    let avg = sum * (w as f32 * h as f32).recip();
                    image = Image::from_fn(w, h, |x, y| polate(image[(x, y)], avg, 1.0 - f));
                }
                Filter::EdgeDetectBase => {
                    image = convolve::filter(
                        &image,
                        [[1.0, 0.0, -1.0], [2.0, 0.0, -2.0], [1.0, 0.0, -1.0]],
                    ).normalize()
                        + convolve::filter(
                            &image,
                            [[1.0, 2.0, 1.0], [0.0, 0.0, 0.0], [-1.0, -2.0, -1.0]],
                        ).normalize()
                        + convolve::filter(
                            &image,
                            [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]],
                        ).normalize()
                        + convolve::filter(
                            &image,
                            [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]],
                        ).normalize();
                    image *= 0.5;
                }
                Filter::FloydSteinbergDither(bits) => {
                    let (w, h) = image.dims();
                    let max = (bits as f32).exp2() - 1.0;
                    let mut img = Image::from_fn(w, h, |_, _| Pixel::default());
                    for x in 0..w {
                        for y in 0..h {
                            let Pixel([r, g, b, a]) = image[(x, y)];
                            let qr = (r * max).round() / max;
                            let qg = (g * max).round() / max;
                            let qb = (b * max).round() / max;
                            img[(x, y)] = Pixel([qr, qg, qb, a]);

                            let mut add = |x: u32, y: u32, xo: u32, yo: u32, p: Pixel| {
                                if 1 <= x + xo && x + xo < w + 1 && 1 <= y + yo && y + yo < h + 1 {
                                    img[(x + xo - 1, y + yo - 1)] += p;
                                }
                            };

                            let p = Pixel([r - qr, g - qg, b - qb, 1.0]);
                            add(x, y, 2, 1, p * (7. / 16.));
                            add(x, y, 0, 2, p * (3. / 16.));
                            add(x, y, 1, 2, p * (5. / 16.));
                            add(x, y, 2, 2, p * (1. / 16.));
                        }
                    }
                    image = img.normalize();
                }
                Filter::Grayscale => {
                    let (w, h) = image.dims();
                    image = Image::from_fn(w, h, |x, y| {
                        let Pixel([r, g, b, a]) = image[(x, y)];
                        let l = 0.3 * r + 0.6 * g + 0.1 * b;
                        Pixel([l, l, l, a])
                    })
                }
                Filter::Output(path) => {
                    let image: RgbaImage = (&image).into();
                    image.save(path)?
                }
                Filter::Pipe(command) => pipe::filter(&image, command)?,
                Filter::Quantitize(bits) => {
                    let (w, h) = image.dims();
                    let max = (bits as f32).exp2() - 1.0;
                    image = Image::from_fn(w, h, |x, y| {
                        let Pixel([r, g, b, a]) = image[(x, y)];
                        let r = (r * max).round() / max;
                        let g = (g * max).round() / max;
                        let b = (b * max).round() / max;
                        Pixel([r, g, b, a])
                    })
                }
                Filter::RandomNoise => {
                    let (w, h) = image.dims();
                    image = Image::from_fn(w, h, |x, y| {
                        let Pixel([r, g, b, a]) = image[(x, y)];
                        let rr: f32 = random();
                        let rg: f32 = random();
                        let rb: f32 = random();
                        Pixel([r + rr * 0.1, g + rg * 0.1, b + rb * 0.1, a])
                    })
                }
                Filter::Rotate(angle) => {
                    image = rotate::filter(&image, sample_mode, angle.to_radians())
                }
                Filter::Sample(mode) => sample_mode = mode,
                Filter::Saturation(f) => {
                    let (w, h) = image.dims();
                    image = Image::from_fn(w, h, |x, y| {
                        transform_as_yuv(image[(x, y)], |y, u, v| (y, u * f, v * f))
                    })
                }
                Filter::Scale(x, y) => image = scale::filter(&image, sample_mode, x, y),
                Filter::Sharpen => {
                    let (w, h) = image.dims();
                    let blur = blur::filter(&image);
                    image = Image::from_fn(w, h, |x, y| polate(image[(x, y)], blur[(x, y)], -1.0));
                }
            }
            let time = start.elapsed();
            debug!("Took {}s{}ms", time.as_secs(), time.subsec_millis());
        }
        Ok(())
    })
}
