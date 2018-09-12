use std::path::PathBuf;

use common::{
    failure::Error,
    glium::{
        index::{NoIndices, PrimitiveType},
        texture::{RawImage2d, Texture2d},
        Program, VertexBuffer,
    },
    glium_sdl2::SDL2Facade,
    image::open as open_image,
    sdl2::Sdl,
};

use vertex::{Vertex, MODEL};

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

pub struct State {
    pub running: bool,
    pub indices: NoIndices,
    pub matrix: [[f32; 4]; 4],
    pub program: Program,
    pub texture: Texture2d,
    pub vbo: VertexBuffer<Vertex>,
}

pub const MATRIX_DEFAULT: [[f32; 4]; 4] = [
    [0.5, 0.0, 0.0, 0.0],
    [0.0, 0.5, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

pub fn on_init(args: Args, _: &mut Sdl, display: &mut SDL2Facade) -> Result<State, Error> {
    const VERT_SHADER_SRC: &str = include_str!("shader.vert");
    const FRAG_SHADER_SRC: &str = include_str!("shader.frag");

    let image = open_image(args.input)?.to_rgba();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = Texture2d::new(&*display, image)?;

    let vbo = VertexBuffer::new(&*display, MODEL)?;
    let indices = NoIndices(PrimitiveType::TrianglesList);
    let program = Program::from_source(&*display, VERT_SHADER_SRC, FRAG_SHADER_SRC, None).unwrap();

    Ok(State {
        running: true,
        program,
        indices,
        matrix: MATRIX_DEFAULT,
        texture,
        vbo,
    })
}
