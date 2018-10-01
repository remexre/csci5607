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

    /// Extracts a channel.
    Channel(usize),

    /// Changes the contrast by a factor.
    Contrast(f32),

    /// The basic convolution used in edge detection.
    EdgeDetectBase,

    /// Grayscale conversion.
    Grayscale,

    /// Shifts the hue by the given amount (0.0, 1.0 == identity).
    HueShift(f32),

    /// Writes out the current image.
    Output(PathBuf),

    /// Pipes the image to a subprocess.
    Pipe(String),

    /// Quantitizes an image to the given maximum value.
    Quantitize(usize),

    /// Sets the sampling mode.
    Sample(SampleMode),

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
                "-edge-detect" => {
                    filters.push(Filter::Blur);
                    filters.push(Filter::Grayscale);
                    filters.push(Filter::EdgeDetectBase);
                    filters.push(Filter::BlackWhiteLuminosity(0.1));
                }
                "-edge-detect-base" => {
                    filters.push(Filter::EdgeDetectBase);
                }
                "-grayscale" => {
                    filters.push(Filter::Grayscale);
                }
                "-hue-shift" => {
                    let amount = iter.next()?.as_ref().parse().ok()?;
                    filters.push(Filter::HueShift(amount));
                }
                "-output" => {
                    let output: PathBuf = iter.next()?.as_ref().into();
                    filters.push(Filter::Output(output));
                }
                "-pipe" => {
                    let command = iter.next()?.as_ref().to_string();
                    filters.push(Filter::Pipe(command));
                }
                "-quantitise" | "-quantitize" | "--quantitise" | "--quantitize" => {
                    let amt: usize = iter.next()?.as_ref().parse().ok()?;
                    if amt == 0 {
                        return None;
                    }
                    filters.push(Filter::Quantitize(amt));
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
