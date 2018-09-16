#[macro_use]
extern crate common;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;

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
    // TODO: Process animation.

    let uniforms = uniform!{
        off: [state.offset.0, state.offset.1],
        rotation: state.rotation,
        scale: state.scale,
        tex0: state.texture.sampled()
            .wrap_function(SamplerWrapFunction::Repeat)
            .magnify_filter(MagnifySamplerFilter::Nearest),
    };

    let mut target = display.draw();

    target.clear_color(1.0, 1.0, 1.0, 1.0);
    target
        .draw(
            &state.vbo,
            &state.indices,
            &state.program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();

    target.finish()?;
    Ok(state.running)
}
