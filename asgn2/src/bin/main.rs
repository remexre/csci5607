extern crate cgmath;
#[macro_use]
extern crate clap;
extern crate image;
#[macro_use]
extern crate log;
extern crate raytracer;
#[cfg(feature = "sdl2")]
extern crate sdl2;

#[cfg(feature = "sdl2")]
mod gui;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use raytracer::Scene;

fn main() {
    let matches = clap_app!(raytracer =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg INPUT: +required "The input file to read.")
        (@arg OUTPUT: -o +takes_value "The output file to write to. Overrides the scene's default.")
        (@arg HEIGHT: -h +takes_value "Overrides the height of the output image.")
        (@arg WIDTH: -w +takes_value "Overrides the width of the output image.")
    ).get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let input = read_file(input_file);

    let mut scene: Scene = input.parse().expect("Failed to parse scene");
    if let Some(h) = matches.value_of("HEIGHT") {
        scene.height = h.parse().expect("Couldn't parse -h argument");
    }
    if let Some(w) = matches.value_of("WIDTH") {
        scene.width = w.parse().expect("Couldn't parse -w argument");
    }

    let path = match matches.value_of("OUTPUT") {
        Some(p) => Some(p.into()),
        None => scene.output_image.clone(),
    };
    if let Some(output_path) = path {
        let image = scene.render();
        image.save(output_path).expect("Failed to write image")
    } else {
        gui::run(scene)
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> String {
    let mut f = File::open(path).expect("Failed to open input file");
    let mut buf = String::new();
    f.read_to_string(&mut buf)
        .expect("Failed to read input file");
    buf
}

#[cfg(not(feature = "sdl2"))]
mod gui {
    use std::process::{Command, Stdio};

    use image::png::PNGEncoder;
    use image::ColorType;
    use raytracer::Scene;

    pub fn run(scene: Scene) {
        let image = scene.render();
        let child = Command::new("display")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start `display'");

        let (width, height) = image.dimensions();
        PNGEncoder::new(&mut child.stdin.unwrap())
            .encode(&image, width, height, ColorType::RGB(8))
            .expect("Failed to write image")
    }
}
