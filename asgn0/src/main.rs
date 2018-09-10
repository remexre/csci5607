extern crate common;
#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;

use std::path::PathBuf;

use common::{
    failure::Error,
    glium::Surface,
    glium_sdl2::SDL2Facade,
    image::{open as open_image, DynamicImage},
    run_wrapper,
    sdl2::{event::Event, keyboard::Scancode, Sdl},
};

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

struct State {
    image: DynamicImage,
    running: bool,
}

fn main() {
    run_wrapper("asgn0", on_init, on_loop, on_event)
}

fn on_init(args: Args, _: &mut Sdl, _: &mut SDL2Facade) -> Result<State, Error> {
    let image = open_image(args.input)?;
    Ok(State {
        image,
        running: true,
    })
}

fn on_loop(state: &mut State, _: &mut Sdl, display: &mut SDL2Facade) -> Result<bool, Error> {
    let mut target = display.draw();

    target.clear_color(0.0, 0.0, 1.0, 0.0);

    target.finish()?;
    Ok(state.running)
}

fn on_event(
    ev: Event,
    state: &mut State,
    _: &mut Sdl,
    display: &mut SDL2Facade,
) -> Result<(), Error> {
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
