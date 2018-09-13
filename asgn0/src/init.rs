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
    nalgebra::Matrix3,
    sdl2::Sdl,
};

use event::DragState;
use vertex::{Vertex, MODEL};

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

lazy_static! {
    pub static ref PROJ_DEFAULT: Matrix3<f32> = {
        let mut mat = Matrix3::zeros();
        mat[(0, 0)] = 1.0;
        mat[(1, 1)] = 1.0;
        mat
    };
    pub static ref PROJ_INV_DEFAULT: Matrix3<f32> = {
        let mut mat = Matrix3::zeros();
        mat[(0, 0)] = 1.0;
        mat[(1, 1)] = 1.0;
        mat
    };
}

pub struct State {
    pub drag: Option<(DragState, f32, f32)>,
    pub running: bool,
    pub indices: NoIndices,
    pub proj: Matrix3<f32>,
    pub proj_inv: Matrix3<f32>,
    pub program: Program,
    pub texture: Texture2d,
    pub vbo: VertexBuffer<Vertex>,
}

impl State {
    /// Resets the projection matrix.
    pub fn reset(&mut self) {
        self.proj = PROJ_DEFAULT.clone();
        self.proj_inv = PROJ_INV_DEFAULT.clone();
    }
}

pub fn on_init(args: Args, _: &mut Sdl, display: &mut SDL2Facade) -> Result<State, Error> {
    const VERT_SHADER_SRC: &str = include_str!("shader.vert");
    const FRAG_SHADER_SRC: &str = include_str!("shader.frag");

    let vbo = VertexBuffer::new(&*display, MODEL)?;
    let indices = NoIndices(PrimitiveType::TrianglesList);
    let program = Program::from_source(&*display, VERT_SHADER_SRC, FRAG_SHADER_SRC, None).unwrap();

    Ok(State {
        drag: None,
        running: true,
        program,
        indices,
        proj: PROJ_DEFAULT.clone(),
        proj_inv: PROJ_INV_DEFAULT.clone(),
        texture: load_texture(args.input, &*display)?,
        vbo,
    })
}
