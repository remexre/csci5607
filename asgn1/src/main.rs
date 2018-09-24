extern crate common;
#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;

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

mod edge_detect;

fn main() {}
