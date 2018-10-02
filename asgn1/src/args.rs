use std::path::PathBuf;

use util::SampleMode;

/// The parsed arguments.
#[derive(Debug)]
pub struct Args {
    pub input: PathBuf,
    pub filters: Vec<Filter>,
}

/// A transformation to apply to an image.
#[derive(Debug)]
pub enum Filter {
    /// Sets the color to white if the luminosity is greater than the given parameter.
    BlackWhiteLuminosity(f32),

    /// A gaussian blur.
    Blur,

    /// Brightens the image.
    Brighten(f32),

    /// Extracts a channel.
    Channel(usize),

    /// Changes the contrast by a factor.
    Contrast(f32),

    /// Crops an image.
    Crop(f32, f32, u32, u32),

    /// The basic convolution used in edge detection.
    EdgeDetectBase,

    /// Changes the number of bits per channel with Floyd-Steinberg dithering.
    FloydSteinbergDither(u8),

    /// Grayscale conversion.
    Grayscale,

    /// Writes out the current image.
    Output(PathBuf),

    /// Pipes the image to a subprocess.
    Pipe(String),

    /// Changes the number of bits per channel.
    Quantitize(u8),

    /// Adds random noise.
    RandomNoise,

    /// Rotates the image counter-clockwise by the given angle.
    Rotate(f32),

    /// Sets the sampling mode.
    Sample(SampleMode),

    /// Changes the saturation by a factor.
    Saturation(f32),

    /// Scales the image.
    Scale(f32, f32),

    /// Sharpens an image by applying a gaussian blur and then extrapolating.
    Sharpen,
}

/// Parses the command-line arguments.
pub fn parse<I: IntoIterator<Item = S>, S: AsRef<str>>(iter: I) -> Option<Args> {
    let mut iter = iter.into_iter().skip(1);

    // Parse out `-input <PATH>`.
    if !iter.next().map(|s| s.as_ref() == "-input")? {
        return None;
    }
    let input: PathBuf = iter.next()?.as_ref().into();

    let mut filters = Vec::new();
    loop {
        if let Some(s) = iter.next() {
            let s = s.as_ref();
            match s {
                "-blur" => {
                    filters.push(Filter::Blur);
                }
                "-brighten" => {
                    let f = iter.next()?.as_ref().parse().ok()?;
                    filters.push(Filter::Brighten(f));
                }
                "-channel" => {
                    let ch = match iter.next()?.as_ref() {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => return None,
                    };
                    filters.push(Filter::Channel(ch));
                }
                "-contrast" => {
                    let factor = iter.next()?.as_ref().parse().ok()?;
                    filters.push(Filter::Contrast(factor));
                }
                "-crop" => {
                    let top = iter.next()?.as_ref().parse().ok()?;
                    let left = iter.next()?.as_ref().parse().ok()?;
                    let width = iter.next()?.as_ref().parse().ok()?;
                    let height = iter.next()?.as_ref().parse().ok()?;
                    filters.push(Filter::Crop(top, left, width, height));
                }
                "-edge-detect" => {
                    filters.push(Filter::Blur);
                    filters.push(Filter::Grayscale);
                    filters.push(Filter::EdgeDetectBase);
                    filters.push(Filter::BlackWhiteLuminosity(0.1));
                }
                "-edge-detect-base" => {
                    filters.push(Filter::EdgeDetectBase);
                }
                "-floyd-steinberg-dither" => {
                    let bits = iter.next()?.as_ref().parse().ok()?;
                    filters.push(Filter::FloydSteinbergDither(bits));
                }
                "-grayscale" => {
                    filters.push(Filter::Grayscale);
                }
                "-output" => {
                    let output: PathBuf = iter.next()?.as_ref().into();
                    filters.push(Filter::Output(output));
                }
                "-pipe" => {
                    let command = iter.next()?.as_ref().to_string();
                    filters.push(Filter::Pipe(command));
                }
                "-quantitise" | "-quantitize" => {
                    let bits: u8 = iter.next()?.as_ref().parse().ok()?;
                    if bits == 0 {
                        return None;
                    }
                    filters.push(Filter::Quantitize(bits));
                }
                "-random-dither" => {
                    let bits = iter.next()?.as_ref().parse().ok()?;
                    if bits == 0 {
                        return None;
                    }
                    filters.push(Filter::RandomNoise);
                    filters.push(Filter::Quantitize(bits));
                }
                "-random-noise" => {
                    filters.push(Filter::RandomNoise);
                }
                "-rotate" => {
                    let angle = iter.next()?.as_ref().parse().ok()?;
                    filters.push(Filter::Rotate(angle));
                }
                "-sample" => {
                    let sample_mode = match iter.next()?.as_ref() {
                        "bilinear" => SampleMode::Bilinear,
                        "gaussian" => SampleMode::Gaussian,
                        "point" => SampleMode::Point,
                        _ => return None,
                    };
                    filters.push(Filter::Sample(sample_mode));
                }
                "-saturation" => {
                    let factor = iter.next()?.as_ref().parse().ok()?;
                    filters.push(Filter::Saturation(factor));
                }
                "-scale" => {
                    let x: f32 = iter.next()?.as_ref().parse().ok()?;
                    let y: f32 = iter.next()?.as_ref().parse().ok()?;
                    filters.push(Filter::Scale(x, y));
                }
                "-sharpen" => {
                    filters.push(Filter::Sharpen);
                }
                _ => break None,
            }
        } else {
            break Some(Args { input, filters });
        }
    }
}
