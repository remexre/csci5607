use std::path::Path;

use failure::Error;
use glium::texture::{RawImage2d, Texture2d};
use glium_sdl2::SDL2Facade;
use image::open as open_image;

pub fn load_texture<P: AsRef<Path>>(path: P, display: &SDL2Facade) -> Result<Texture2d, Error> {
    let image = open_image(path)?.to_rgba();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = Texture2d::new(&*display, image)?;
    Ok(texture)
}
