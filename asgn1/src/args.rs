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
    /// Edge detection.
    EdgeDetect,

    /// Writes out the current image.
    Output(PathBuf),

    /// Pipes the image to a subprocess.
    Pipe(String),

    /// Sets the sampling mode.
    Sample(SampleMode),

    /// Scales the image.
    Scale(f32, f32),
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
                "-edge-detect" => {
                    filters.push(Filter::EdgeDetect);
                }
                "-output" => {
                    let output: PathBuf = iter.next()?.as_ref().into();
                    filters.push(Filter::Output(output));
                }
                "-pipe" => {
                    let command = iter.next()?.as_ref().to_string();
                    filters.push(Filter::Pipe(command));
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
                _ => break None,
            }
        } else {
            break Some(Args { input, filters });
        }
    }
}
