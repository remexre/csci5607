use image::{Pixel, Rgb};

/// The material a Renderable is made of.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Material {
    /// The ambient color of the material.
    pub ambient: Rgb<f32>,

    /// The diffuse color of the material.
    pub diffuse: Rgb<f32>,

    /// The specular color of the material.
    pub specular: Rgb<f32>,

    /// The phong cosine power for specular highlights.
    pub phong: f32,

    /// The transmissive color of the material.
    pub transmissive: Rgb<f32>,

    /// The index of refraction of the material.
    pub ior: f32,
}

impl Default for Material {
    fn default() -> Material {
        Material {
            ambient: Pixel::from_channels(0.0, 0.0, 0.0, 1.0),
            diffuse: Pixel::from_channels(1.0, 1.0, 1.0, 1.0),
            specular: Pixel::from_channels(0.0, 0.0, 0.0, 1.0),
            phong: 5.0,
            transmissive: Pixel::from_channels(0.0, 0.0, 0.0, 1.0),
            ior: 1.0,
        }
    }
}
