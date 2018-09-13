use common::{
    failure::Error,
    glium_sdl2::SDL2Facade,
    sdl2::{event::Event, keyboard::Scancode, mouse::MouseButton, Sdl},
};

use init::State;

#[derive(Clone, Copy, Debug)]
pub enum DragState {
    Corner,
    Edge,
    Middle,
}

pub fn on_event(
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
            Some(Scancode::R) => {
                state.reset();
            }
            _ => {}
        },
        Event::MouseButtonDown {
            x, y, mouse_btn, ..
        } => if mouse_btn == MouseButton::Left {
            if let Some((x, y)) = transform_coords(x, y, display.window().size()) {
                handle_mouse(x, y, MouseActionType::ButtonDown, state);
            }
        },
        Event::MouseButtonUp {
            x, y, mouse_btn, ..
        } => {
            if mouse_btn == MouseButton::Left {
                if let Some((x, y)) = transform_coords(x, y, display.window().size()) {
                    handle_mouse(x, y, MouseActionType::ButtonUp, state);
                }
            }
        }
        Event::MouseMotion { x, y, .. } => info!("Mouse Moved ({}, {})", x, y),
        _ => debug!("Unhandled event {:?}", ev),
    }
    Ok(())
}

enum MouseActionType {
    ButtonDown,
    ButtonUp,
    Motion,
}

fn handle_mouse(x: f32, y: f32, ty: MouseActionType, state: &mut State) {
    error!("TODO handle_mouse({}, {}, _)", x, y)
}

/// Transforms screen coordinates to OpenGL coordinates (i.e. (0, 0) at center, (1, 1) at top
/// right). If the coordinates are invalid, returns None.
fn transform_coords(x: i32, y: i32, max: (u32, u32)) -> Option<(f32, f32)> {
    if x < 0 || y < 0 {
        return None;
    }

    let x = x as f32;
    let y = y as f32;
    let max_x = max.0 as f32;
    let max_y = max.1 as f32;

    let x = (2.0 * x / max_x) - 1.0;
    let y = 1.0 - (2.0 * y / max_y);

    if -1.0 <= x && x <= 1.0 && -1.0 <= y && y <= 1.0 {
        Some((x, y))
    } else {
        None
    }
}
