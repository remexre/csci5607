extern crate common;
#[macro_use]
extern crate log;

mod args;
mod scale;
mod util;

use std::env::args;
use std::process::exit;
use std::time::Instant;

use common::{image::open as open_image, run_err};

use args::Filter;

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
    -scale X Y      Scales the image by the given factor in the X and Y directions.
    -output PATH    Writes the current state of the image to the given path.

NOTES:
    Filters are applied one after the other, left to right. The upper-left corner is the origin,
    with the X axis going right and the Y axis going down."#;
            eprintln!("{}", USAGE);
            exit(-1);
        }
    };

    run_err(move || {
        let mut image = open_image(args.input)?.to_rgba();
        for filter in args.filters {
            debug!("Applying {:?}...", filter);
            let start = Instant::now();
            match filter {
                Filter::Output(path) => image.save(path)?,
                Filter::Scale(x, y) => scale::filter(&mut image, x, y),
            }
            let time = start.elapsed();
            debug!("Took {}s{}ms", time.as_secs(), time.subsec_millis());
        }
        Ok(())
    })
}
