//! A parser for the file format used for scenes.

mod math;
mod nom;
#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::str::FromStr;

use cgmath::num_traits::clamp;
use cgmath::{InnerSpace, Vector3};
use image::Rgb;
use nom::{Err, IResult};

use light::{Directional, DynamicLight, Point, Spot};
use material::Material;
use renderable::{DynamicRenderable, Plane, Sphere, Triangle};
use scene::Scene;

impl FromStr for Scene {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Scene, ParseError> {
        let lines = match nom::parse_lines(input) {
            IResult::Done("", lines) => Ok(lines),
            IResult::Done(rest, _) => Err(ParseError::Unparsed(rest.to_string())),
            IResult::Incomplete(_) => unreachable!(),
            IResult::Error(e) => Err(e.into()),
        }?;

        let mut scene = Scene::default();
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut material = Material::default();
        for line in lines {
            match line {
                Line::AmbientLight(r, g, b) => scene.ambient_light = convert_color(r, g, b),
                Line::Background(r, g, b) => scene.background = convert_color(r, g, b),
                Line::Camera(px, py, pz, dx, dy, dz, ux, uy, uz, ha) => {
                    scene.camera_position = Vector3::new(px, py, pz);
                    scene.camera_direction = Vector3::new(dx, dy, dz).normalize();
                    scene.camera_up = Vector3::new(ux, uy, uz).normalize();
                    scene.camera_half_angle_tan = ha.to_radians().tan();
                }
                Line::DirectionalLight(r, g, b, x, y, z, i) => {
                    scene.lights.push(DynamicLight::Directional(Directional {
                        color: convert_color(r, g, b),
                        direction: Vector3::new(x, y, z),
                        intensity: i,
                    }));
                }
                Line::Material(ar, ag, ab, dr, dg, db, sr, sg, sb, ns, tr, tg, tb, ior) => {
                    material = Material {
                        ambient: convert_color(ar, ag, ab),
                        diffuse: convert_color(dr, dg, db),
                        specular: convert_color(sr, sg, sb),
                        phong: ns,
                        transmissive: convert_color(tr, tg, tb),
                        ior,
                    };
                }
                Line::MaxDepth(n) => scene.max_collisions = n,
                Line::MaxNormals(_) | Line::MaxVertices(_) => {
                    warn!("Ignoring {:?}", line);
                }
                Line::Normal(x, y, z) => normals.push(Vector3::new(x, y, z)),
                Line::NormalTriangle(v1, v2, v3, n1, n2, n3) => {
                    // TODO: This could *greatly* benefit from macros.
                    let v1 = if let Some(&v) = vertices.get(v1) {
                        v
                    } else {
                        return Err(ParseError::NoSuchVertex(line, v1));
                    };
                    let v2 = if let Some(&v) = vertices.get(v2) {
                        v
                    } else {
                        return Err(ParseError::NoSuchVertex(line, v2));
                    };
                    let v3 = if let Some(&v) = vertices.get(v3) {
                        v
                    } else {
                        return Err(ParseError::NoSuchVertex(line, v3));
                    };
                    let n1 = if let Some(&n) = normals.get(n1) {
                        n
                    } else {
                        return Err(ParseError::NoSuchNormal(line, n1));
                    };
                    let n2 = if let Some(&n) = normals.get(n2) {
                        n
                    } else {
                        return Err(ParseError::NoSuchNormal(line, n2));
                    };
                    let n3 = if let Some(&n) = normals.get(n3) {
                        n
                    } else {
                        return Err(ParseError::NoSuchNormal(line, n3));
                    };
                    let tri = Triangle {
                        vertices: (v1, v2, v3),
                        normal: math::normal_from_normals(v1, v2, v3, n1, n2, n3),
                        material: material.clone(),
                    };
                    scene.objects.push(DynamicRenderable::Triangle(tri));
                }
                Line::Output(path) => scene.output_image = Some(path),
                Line::PointLight(r, g, b, x, y, z, i) => {
                    scene.lights.push(DynamicLight::Point(Point {
                        color: convert_color(r, g, b),
                        position: Vector3::new(x, y, z),
                        intensity: i,
                    }));
                }
                Line::Resolution(w, h) => {
                    scene.height = h;
                    scene.width = w;
                }
                Line::Plane(px, py, pz, nx, ny, nz) => {
                    scene.objects.push(DynamicRenderable::Plane(Plane {
                        point: Vector3::new(px, py, pz),
                        normal: Vector3::new(nx, ny, nz),
                        material: material.clone(),
                    }));
                }
                Line::Sphere(x, y, z, r) => {
                    scene.objects.push(DynamicRenderable::Sphere(Sphere {
                        position: Vector3::new(x, y, z),
                        radius: r,
                        material: material.clone(),
                    }));
                }
                Line::SpotLight(r, g, b, px, py, pz, dx, dy, dz, a1, a2, i) => {
                    scene.lights.push(DynamicLight::Spot(Spot {
                        color: convert_color(r, g, b),
                        position: Vector3::new(px, py, pz),
                        direction: Vector3::new(dx, dy, dz),
                        intensity: i,
                        falloff_angle: a1,
                        max_angle: a2,
                    }));
                }
                Line::Triangle(v1, v2, v3) => {
                    let v1 = if let Some(&v) = vertices.get(v1) {
                        v
                    } else {
                        return Err(ParseError::NoSuchVertex(line, v1));
                    };
                    let v2 = if let Some(&v) = vertices.get(v2) {
                        v
                    } else {
                        return Err(ParseError::NoSuchVertex(line, v2));
                    };
                    let v3 = if let Some(&v) = vertices.get(v3) {
                        v
                    } else {
                        return Err(ParseError::NoSuchVertex(line, v3));
                    };
                    let tri = Triangle {
                        vertices: (v1, v2, v3),
                        normal: math::normal_from_points(v1, v2, v3),
                        material: material.clone(),
                    };
                    scene.objects.push(DynamicRenderable::Triangle(tri));
                }
                Line::Vertex(x, y, z) => vertices.push(Vector3::new(x, y, z)),
            }
        }
        Ok(scene)
    }
}

fn convert_color(r: f32, g: f32, b: f32) -> Rgb<f32> {
    Rgb {
        data: [clamp(r, 0.0, 1.0), clamp(g, 0.0, 1.0), clamp(b, 0.0, 1.0)],
    }
}

/// A single (non-comment) line of the scene file.
///
/// You know, this could actually be defined as a monad...
/// You'd have a parser monad that is MonadException and MonadReader, then a
/// line monad that's MonadState. Hm. Food for thought.
///
/// Also, you could define the `[Line] -> Scene` function as a fold, too.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub enum Line {
    AmbientLight(f32, f32, f32),
    Background(f32, f32, f32),
    Camera(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32),
    DirectionalLight(f32, f32, f32, f32, f32, f32, f32),
    Material(
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
    ),
    MaxDepth(usize),
    MaxNormals(usize),
    MaxVertices(usize),
    Normal(f32, f32, f32),
    NormalTriangle(usize, usize, usize, usize, usize, usize),
    Output(PathBuf),
    Plane(f32, f32, f32, f32, f32, f32),
    PointLight(f32, f32, f32, f32, f32, f32, f32),
    Resolution(u32, u32),
    Sphere(f32, f32, f32, f32),
    SpotLight(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32),
    Triangle(usize, usize, usize),
    Vertex(f32, f32, f32),
}

/// An error while parsing.
#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    /// An error returned by a Nom parser.
    Nom(Err),

    /// A non-existent normal was referenced.
    NoSuchNormal(Line, usize),

    /// A non-existent vertex was referenced.
    NoSuchVertex(Line, usize),

    /// There was unparsed input left after parsing the string.
    Unparsed(String),
}

impl From<Err> for ParseError {
    fn from(e: Err) -> ParseError {
        ParseError::Nom(e)
    }
}
