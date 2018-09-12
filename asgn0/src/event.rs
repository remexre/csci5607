use common::{
    failure::Error,
    glium_sdl2::SDL2Facade,
    sdl2::{event::Event, keyboard::Scancode, Sdl},
};

use init::State;

pub fn on_event(
    ev: Event,
    state: &mut State,
    _: &mut Sdl,
    _: &mut SDL2Facade,
) -> Result<(), Error> {
    match ev {
        Event::KeyDown { scancode, .. } => match scancode {
            Some(Scancode::Q) => {
                state.running = false;
            }
            _ => {}
        },
        Event::MouseButtonDown {
            x, y, mouse_btn, ..
        } => info!("Mouse Down {:?} ({}, {})", mouse_btn, x, y),
        Event::MouseButtonUp {
            x, y, mouse_btn, ..
        } => info!("Mouse Up {:?} ({}, {})", mouse_btn, x, y),
        Event::MouseMotion { x, y, .. } => info!("Mouse Moved ({}, {})", x, y),
        _ => debug!("Unknown event {:?}", ev),
    }
    Ok(())
}
