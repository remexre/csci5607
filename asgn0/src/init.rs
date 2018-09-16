use std::path::PathBuf;

use common::{
    failure::Error,
    glium::{
        index::{NoIndices, PrimitiveType},
        texture::Texture2d,
        Program, VertexBuffer,
    },
    glium_sdl2::SDL2Facade,
    helpers::load_texture,
    sdl2::Sdl,
};

use event::SquarePart;
use vertex::{Vertex, MODEL};

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

pub struct State {
    pub animate: bool,
    pub drag: Option<(SquarePart, f32)>,
    pub running: bool,

    pub aspect_ratio: f32,
    pub indices: NoIndices,
    pub program: Program,
    pub texture: Texture2d,
    pub vbo: VertexBuffer<Vertex>,

    pub offset: (f32, f32),
    pub rotation: f32,
    pub scale: f32,
}

impl State {
    /// Resets the projection matrix.
    pub fn reset(&mut self) {
        self.drag = None;
        self.offset = (0.0, 0.0);
        self.rotation = 0.0;
        self.scale = 0.5;
    }
}

pub fn on_init(args: Args, _: &mut Sdl, display: &mut SDL2Facade) -> Result<State, Error> {
    const VERT_SHADER_SRC: &str = include_str!("shader.vert");
    const FRAG_SHADER_SRC: &str = include_str!("shader.frag");

    let vbo = VertexBuffer::new(&*display, MODEL)?;
    let indices = NoIndices(PrimitiveType::TrianglesList);
    let program = Program::from_source(&*display, VERT_SHADER_SRC, FRAG_SHADER_SRC, None).unwrap();

    let (w, h) = display.window().size();
    Ok(State {
        animate: false,
        drag: None,
        running: true,

        aspect_ratio: w as f32 / h as f32,
        indices,
        program,
        texture: load_texture(args.input, &*display)?,
        vbo,

        offset: (0.0, 0.0),
        rotation: 0.0,
        scale: 0.5,
    })
}
