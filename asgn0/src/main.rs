extern crate common;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;

mod bounds;
mod event;
mod init;
mod vertex;

use common::{
    failure::Error,
    glium::{
        uniforms::{MagnifySamplerFilter, SamplerWrapFunction},
        Surface,
    },
    glium_sdl2::SDL2Facade,
    run_wrapper,
    sdl2::Sdl,
};

use event::on_event;
use init::{on_init, State};

fn main() {
    run_wrapper("asgn0", on_init, on_loop, on_event)
}

fn on_loop(state: &mut State, _: &mut Sdl, display: &mut SDL2Facade) -> Result<bool, Error> {
    let uniforms = uniform!{
        matrix: state.matrix,
        tex0: state.texture.sampled()
            .wrap_function(SamplerWrapFunction::Repeat)
            .magnify_filter(MagnifySamplerFilter::Nearest),
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
