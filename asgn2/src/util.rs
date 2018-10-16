//! Simple utilities.

use cgmath::num_traits::clamp;
use cgmath::Vector3;
use float_ord::FloatOrd;
use image::Rgb;

/// The color black.
pub const BLACK: Rgb<f32> = Rgb {
    data: [0.0, 0.0, 0.0],
};

/// Blends two colors, returning a new one.
pub fn add_colors(a: Rgb<f32>, b: Rgb<f32>) -> Rgb<f32> {
    normalize_color_down(Rgb {
        data: [
            a.data[0] + b.data[0],
            a.data[1] + b.data[1],
            a.data[2] + b.data[2],
        ],
    })
}

/// Clamps a color to [0.0, 1.0].
pub fn clamp_color(c: Rgb<f32>) -> Rgb<f32> {
    Rgb {
        data: [
            clamp(c.data[0], 0.0, 1.0),
            clamp(c.data[1], 0.0, 1.0),
            clamp(c.data[2], 0.0, 1.0),
        ],
    }
}

/// Component-wise multiplies two colors.
pub fn mul_colors(a: Rgb<f32>, b: Rgb<f32>) -> Rgb<f32> {
    Rgb {
        data: [
            a.data[0] * b.data[0],
            a.data[1] * b.data[1],
            a.data[2] * b.data[2],
        ],
    }
}

/// Normalizes a color in the range [0, Inf) to [0.0, 1.0].
pub fn normalize_color_down(c: Rgb<f32>) -> Rgb<f32> {
    let c = vector_from_rgb(c);
    let n = [1.0, c.x, c.y, c.z]
        .into_iter()
        .cloned()
        .map(FloatOrd)
        .max()
        .unwrap()
        .0;
    rgb_from_vector(c / n)
}

/// Multiplies a scalar by a color.
pub fn scale_color(c: Rgb<f32>, n: f32) -> Rgb<f32> {
    Rgb {
        data: [c.data[0] * n, c.data[1] * n, c.data[2] * n],
    }
}

/// Converts a vector to a RGB color.
pub fn rgb_from_vector(v: Vector3<f32>) -> Rgb<f32> {
    Rgb {
        data: [v.x, v.y, v.z],
    }
}

/// Converts an RGB color to a vector.
pub fn vector_from_rgb(c: Rgb<f32>) -> Vector3<f32> {
    Vector3 {
        x: c.data[0],
        y: c.data[1],
        z: c.data[2],
    }
}
