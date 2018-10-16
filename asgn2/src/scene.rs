use std::mem::forget;
use std::path::PathBuf;

use cgmath::Vector3;
use image::{Pixel, Rgb, RgbImage};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use light::DynamicLight;
use renderable::DynamicRenderable;

/// A single renderable scene.
#[derive(Clone, Debug, PartialEq)]
pub struct Scene {
    /// The position the camera is at.
    pub camera_position: Vector3<f32>,

    /// The direction the camera is facing. Normalized.
    pub camera_direction: Vector3<f32>,

    /// The "up vector" for the camera. Normalized.
    pub camera_up: Vector3<f32>,

    /// The tangent of one half of the vertical angle of the view frustrum.
    pub camera_half_angle_tan: f32,

    /// The width of the output image.
    pub width: u32,

    /// The height of the output image.
    pub height: u32,

    /// The file to save the output image to.
    pub output_image: Option<PathBuf>,

    /// The ambient light color.
    pub ambient_light: Rgb<f32>,

    /// The background light color.
    pub background: Rgb<f32>,

    /// The lights in the scene.
    pub lights: Vec<DynamicLight>,

    /// The objects in the scene.
    pub objects: Vec<DynamicRenderable>,

    /// The maximum number of collisions to process.
    ///
    /// TODO: What happens when this is triggered.
    pub max_collisions: usize,
}

impl Scene {
    /// Renders the scene to an image.
    pub fn render(&self) -> RgbImage {
        let rays = self.render_rays();
        let buf = unsafe {
            let len = rays.len();
            let cap = rays.capacity();
            let ptr = rays.as_ptr();
            forget(rays);
            Vec::from_raw_parts(ptr as *mut u8, len * 3, cap * 3)
        };
        RgbImage::from_raw(self.width, self.height, buf).unwrap()
    }

    #[cfg(feature = "rayon")]
    fn render_rays(&self) -> Vec<[u8; 3]> {
        (0..self.height)
            .into_par_iter()
            .flat_map(|y| (0..self.width).into_par_iter().map(move |x| (x, y)))
            .map(|(x, y)| self.trace_pixel(x, y))
            .collect()
    }
    #[cfg(not(feature = "rayon"))]
    fn render_rays(&self) -> Vec<[u8; 3]> {
        (0..self.height)
            .into_iter()
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .map(|(x, y)| self.trace_pixel(x, y))
            .collect()
    }
}

impl Default for Scene {
    fn default() -> Scene {
        Scene {
            camera_position: Vector3::new(0.0, 0.0, 0.0),
            camera_direction: Vector3::new(0.0, 0.0, 1.0),
            camera_up: Vector3::new(0.0, 1.0, 0.0),
            camera_half_angle_tan: 1.0,
            width: 640,
            height: 480,
            output_image: None,
            ambient_light: Pixel::from_channels(0.0, 0.0, 0.0, 1.0),
            background: Pixel::from_channels(0.0, 0.0, 0.0, 1.0),
            lights: Vec::new(),
            objects: Vec::new(),
            max_collisions: 5,
        }
    }
}
