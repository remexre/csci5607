//! A simple raytracer in Rust.

#![warn(missing_docs)]

extern crate cgmath;
extern crate float_ord;
extern crate image;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;
#[cfg(feature = "rayon")]
extern crate rayon;

pub mod light;
mod material;
pub mod parser;
mod ray;
pub mod renderable;
mod scene;
mod trace;
pub mod util;

pub use material::Material;
pub use ray::Ray;
pub use scene::Scene;
