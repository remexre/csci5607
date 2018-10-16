#![allow(missing_docs)]

use std::path::PathBuf;

use nom::digit;

use super::Line;

named!(pub parse_lines(&str) -> Vec<Line>, complete!(map!(
    many0!(parse_line),
    |v| v.into_iter().filter_map(|x| x).collect())));

named!(pub parse_line(&str) -> Option<Line>, ws!(alt!(
    comment | map!(parse_line_no_comment, Some))));
named!(comment(&str) -> Option<Line>, do_parse!(
    tag!("#") >>
    take_until_s!("\n") >>
    (None)));

/// Parses a single non-comment line.
named!(pub parse_line_no_comment(&str) -> Line, alt!(
    camera | film_resolution | output_image |
    max_vertices | max_normals | vertex | normal | triangle | normal_triangle |
    plane | sphere | background |
    material |
    directional_light | point_light | spot_light | ambient_light |
    max_depth));

named!(camera(&str) -> Line, ws!(do_parse!(
    tag_s!("camera") >>
    px: f32_s >> py: f32_s >> pz: f32_s >>
    dx: f32_s >> dy: f32_s >> dz: f32_s >>
    ux: f32_s >> uy: f32_s >> uz: f32_s >>
    ha: f32_s >>
    (Line::Camera(px, py, pz, dx, dy, dz, ux, uy, uz, ha)))));
named!(film_resolution(&str) -> Line, ws!(do_parse!(
    tag_s!("film_resolution") >> width: u32_s >> height: u32_s >>
    (Line::Resolution(width, height)))));
named!(output_image(&str) -> Line, ws!(do_parse!(
    tag_s!("output_image") >> path: ws!(take_until_s!("\n")) >>
    (Line::Output(PathBuf::from(path))))));
named!(max_vertices(&str) -> Line, ws!(do_parse!(
    tag_s!("max_vertices") >> n: usize_s >>
    (Line::MaxVertices(n)))));
named!(max_normals(&str) -> Line, ws!(do_parse!(
    tag_s!("max_normals") >> n: usize_s >>
    (Line::MaxNormals(n)))));
named!(vertex(&str) -> Line, ws!(do_parse!(
    tag_s!("vertex") >> x: f32_s >> y: f32_s >> z: f32_s >>
    (Line::Vertex(x, y, z)))));
named!(normal(&str) -> Line, ws!(do_parse!(
    tag_s!("normal") >> x: f32_s >> y: f32_s >> z: f32_s >>
    (Line::Normal(x, y, z)))));
named!(triangle(&str) -> Line, ws!(do_parse!(
    tag_s!("triangle") >>
    v1: usize_s >> v2: usize_s >> v3: usize_s >>
    (Line::Triangle(v1, v2, v3)))));
named!(normal_triangle(&str) -> Line, ws!(do_parse!(
    tag_s!("normal_triangle") >>
    v1: usize_s >> v2: usize_s >> v3: usize_s >>
    n1: usize_s >> n2: usize_s >> n3: usize_s >>
    (Line::NormalTriangle(v1, v2, v3, n1, n2, n3)))));
named!(plane(&str) -> Line, ws!(do_parse!(
    tag_s!("plane") >>
    px: f32_s >> py: f32_s >> pz: f32_s >>
    nx: f32_s >> ny: f32_s >> nz: f32_s >>
    (Line::Plane(px, py, pz, nx, ny, nz)))));
named!(sphere(&str) -> Line, ws!(do_parse!(
    tag_s!("sphere") >> x: f32_s >> y: f32_s >> z: f32_s >> r: f32_s >>
    (Line::Sphere(x, y, z, r)))));
named!(background(&str) -> Line, ws!(do_parse!(
    tag_s!("background") >> r: f32_s >> g: f32_s >> b: f32_s >>
    (Line::Background(r, g, b)))));
named!(material(&str) -> Line, ws!(do_parse!(
    tag_s!("material") >>
    ar: f32_s >> ag: f32_s >> ab: f32_s >>
    dr: f32_s >> dg: f32_s >> db: f32_s >>
    sr: f32_s >> sg: f32_s >> sb: f32_s >> ns: f32_s >>
    tr: f32_s >> tg: f32_s >> tb: f32_s >> ior: f32_s >>
    (Line::Material(ar, ag, ab, dr, dg, db, sr, sg, sb, ns, tr, tg, tb, ior)))));
named!(directional_light(&str) -> Line, ws!(do_parse!(
    tag_s!("directional_light") >>
    r: f32_s >> g: f32_s >> b: f32_s >>
    x: f32_s >> y: f32_s >> z: f32_s >>
    i: f32_s >>
    (Line::DirectionalLight(r, g, b, x, y, z, i)))));
named!(point_light(&str) -> Line, ws!(do_parse!(
    tag_s!("point_light") >>
    r: f32_s >> g: f32_s >> b: f32_s >>
    x: f32_s >> y: f32_s >> z: f32_s >>
    i: f32_s >>
    (Line::PointLight(r, g, b, x, y, z, i)))));
named!(spot_light(&str) -> Line, ws!(do_parse!(
    tag_s!("spot_light") >>
    r: f32_s >> g: f32_s >> b: f32_s >>
    px: f32_s >> py: f32_s >> pz: f32_s >>
    dx: f32_s >> dy: f32_s >> dz: f32_s >>
    a1: f32_s >> a2: f32_s >>
    i: f32_s >>
    (Line::SpotLight(r, g, b, px, py, pz, dx, dy, dz, a1, a2, i)))));
named!(ambient_light(&str) -> Line, ws!(do_parse!(
    tag_s!("ambient_light") >>
    r: f32_s >> g: f32_s >> b: f32_s >>
    (Line::AmbientLight(r, g, b)))));
named!(max_depth(&str) -> Line, ws!(do_parse!(
    tag_s!("max_depth") >> d: usize_s >>
    (Line::MaxDepth(d)))));

named!(f32_s(&str) -> f32, flat_map!(
    recognize!(tuple!(
        opt!(alt!(tag_s!("+") | tag_s!("-"))),
        alt!(
            delimited!(digit, tag_s!("."), opt!(digit)) |
            delimited!(opt!(digit), tag_s!("."), digit) |
            digit
        )
    )),
    parse_to!(f32)
));
named!(u32_s(&str) -> u32, flat_map!(recognize!(digit), parse_to!(u32)));
named!(usize_s(&str) -> usize, flat_map!(recognize!(digit), parse_to!(usize)));
