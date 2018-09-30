extern crate common;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

mod args;
mod convolve;
mod pipe;
mod scale;
mod util;

use std::env::args;
use std::process::exit;
use std::time::Instant;

use common::{
    image::{open as open_image, RgbaImage},
    run_err,
};

use args::Filter;

use util::{transform_as_hsv, Image, Pixel, SampleMode};

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
    -channel CHANNEL     Extracts a single channel (red, green, blue).
    -edge-detect         Detects edges.
    -edge-detect-base    Runs the edge-detection filter at the center of the -edge-detect filter.
    -grayscale           Transforms the image to grayscale.
    -output PATH         Writes the current state of the image to the given path.
    -pipe COMMAND        Pipes the image as a JPEG to the given command.
    -sample bilinear     Sets sampling mode to bilinear.
    -sample gaussian     Sets sampling mode to Gaussian.
    -sample point        Sets sampling mode to point.
    -scale X Y           Scales the image by the given factor in the X and Y directions.

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
                    image = convolve::filter(
                        &image,
                        [
                            [1.0 / 16.0, 1.0 / 8.0, 1.0 / 16.0],
                            [1.0 / 8.0, 1.0 / 4.0, 1.0 / 8.0],
                            [1.0 / 16.0, 1.0 / 8.0, 1.0 / 16.0],
                        ],
                    )
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
                Filter::Grayscale => {
                    let (w, h) = image.dims();
                    image = Image::from_fn(w, h, |x, y| {
                        let Pixel([r, g, b, a]) = image[(x, y)];
                        let l = 0.3 * r + 0.6 * g + 0.1 * b;
                        Pixel([l, l, l, a])
                    })
                }
                Filter::HueShift(amt) => {
                    let (w, h) = image.dims();
                    image = Image::from_fn(w, h, |x, y| {
                        transform_as_hsv(image[(x, y)], |h, s, v| (h + amt, s, v))
                    })
                }
                Filter::Output(path) => {
                    let image: RgbaImage = (&image).into();
                    image.save(path)?
                }
                Filter::Pipe(command) => pipe::filter(&image, command)?,
                Filter::Quantitize(max) => {
                    let (w, h) = image.dims();
                    image = Image::from_fn(w, h, |x, y| {
                        let max = max as f32;
                        let Pixel([r, g, b, a]) = image[(x, y)];
                        let r = (r * max).round() / max;
                        let g = (g * max).round() / max;
                        let b = (b * max).round() / max;
                        Pixel([r, g, b, a])
                    })
                }
                Filter::Sample(mode) => sample_mode = mode,
                Filter::Scale(x, y) => image = scale::filter(&image, sample_mode, x, y),
            }
            let time = start.elapsed();
            debug!("Took {}s{}ms", time.as_secs(), time.subsec_millis());
        }
        Ok(())
    })
}
