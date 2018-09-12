extern crate common;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;

use std::path::PathBuf;

use common::{
    failure::Error,
    glium::{
        index::{NoIndices, PrimitiveType},
        texture::Texture2d,
        Program, Surface, VertexBuffer,
    },
    glium_sdl2::SDL2Facade,
    image::open as open_image,
    run_wrapper,
    sdl2::{event::Event, keyboard::Scancode, Sdl},
};

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

struct State {
    running: bool,
    indices: NoIndices,
    program: Program,
    texture: Texture2d,
    vbo: VertexBuffer<Vertex>,
}

#[derive(Clone, Copy, Debug)]
struct Vertex {
    pos: [f32; 2],
    //color: [f32; 3],
    uv: [f32; 2],
}

implement_vertex!(Vertex, pos, uv);

static VERTICES: &[Vertex] = &[
    Vertex {
        pos: [-1.0, -1.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        pos: [1.0, 1.0],
        uv: [1.0, 1.0],
    },
    Vertex {
        pos: [1.0, 1.0],
        uv: [1.0, 1.0],
    },
    Vertex {
        pos: [1.0, -1.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        pos: [-1.0, -1.0],
        uv: [0.0, 0.0],
    },
];

fn main() {
    run_wrapper("asgn0", on_init, on_loop, on_event)
}

fn on_init(args: Args, _: &mut Sdl, display: &mut SDL2Facade) -> Result<State, Error> {
    const VERT_SHADER_SRC: &str = include_str!("shader.vert");
    const FRAG_SHADER_SRC: &str = include_str!("shader.frag");

    let image = open_image(args.input)?.to_rgba();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&*display, image)?;

    let vbo = VertexBuffer::new(&*display, VERTICES)?;
    let indices = NoIndices(PrimitiveType::TrianglesList);
    let program = Program::from_source(&*display, VERT_SHADER_SRC, FRAG_SHADER_SRC, None).unwrap();

    Ok(State {
        running: true,
        program,
        indices,
        texture,
        vbo,
    })
}

fn on_loop(state: &mut State, _: &mut Sdl, display: &mut SDL2Facade) -> Result<bool, Error> {
    let uniforms = uniform!{
        tex0: &state.texture,
    };

    let mut target = display.draw();

    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(
        &state.vbo,
        &state.indices,
        &state.program,
        &uniforms,
        &Default::default(),
    )?;

    target.finish()?;
    Ok(state.running)
}

fn on_event(ev: Event, state: &mut State, _: &mut Sdl, _: &mut SDL2Facade) -> Result<(), Error> {
    match ev {
        Event::KeyDown { scancode, .. } => match scancode {
            Some(Scancode::Q) => {
                state.running = false;
            }
            _ => {}
        },
        _ => debug!("Unknown event {:?}", ev),
    }
    Ok(())
}
