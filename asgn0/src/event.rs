use std::f32::consts::PI;

use common::{
    failure::Error,
    glium_sdl2::SDL2Facade,
    nalgebra::{Matrix3, RowVector3},
    sdl2::{event::Event, keyboard::Scancode, mouse::MouseButton, Sdl},
};

use init::State;

#[derive(Clone, Copy, Debug)]
pub enum AxisPart {
    Minus,
    Center,
    Plus,
}

impl AxisPart {
    fn from_coord(x: f32) -> Option<AxisPart> {
        if -1.0 <= x && x <= (SquarePart::EDGE_WIDTH - 1.0) {
            Some(AxisPart::Minus)
        } else if (1.0 - SquarePart::EDGE_WIDTH) <= x && x <= 1.0 {
            Some(AxisPart::Plus)
        } else if (SquarePart::EDGE_WIDTH - 1.0) <= x && x <= (1.0 - SquarePart::EDGE_WIDTH) {
            Some(AxisPart::Center)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SquarePart {
    Corner,
    Edge,
    Middle,
}

impl SquarePart {
    /// The width (in square coordinates, i.e. where the square is 2.0 units high and wide, and
    /// centered at (0.0, 0.0)).
    pub const EDGE_WIDTH: f32 = 0.5;

    /// Gets the part of the square corresponding to the given coordinates on the square.
    pub fn from_coords(x: f32, y: f32) -> Option<(SquarePart, f32)> {
        let x_edge = AxisPart::from_coord(x)?;
        let y_edge = AxisPart::from_coord(y)?;

        let part = match (x_edge, y_edge) {
            (AxisPart::Center, AxisPart::Center) => SquarePart::Middle,
            (_, AxisPart::Center) => SquarePart::Edge,
            (AxisPart::Center, _) => SquarePart::Edge,
            _ => SquarePart::Corner,
        };
        let angle = (y / x).atan();
        Some((part, angle))
    }
}

pub fn on_event(
    ev: Event,
    state: &mut State,
    _: &mut Sdl,
    display: &mut SDL2Facade,
) -> Result<(), Error> {
    match ev {
        Event::KeyDown { scancode, .. } => match scancode {
            Some(Scancode::A) => state.animate = !state.animate,
            Some(Scancode::Q) => state.running = false,
            Some(Scancode::R) => state.reset(),
            Some(Scancode::Minus) => state.scale *= 0.9,
            Some(Scancode::Equals) => state.scale *= 1.1,
            Some(Scancode::LeftBracket) => state.rotation += PI / 12.0,
            Some(Scancode::RightBracket) => state.rotation -= PI / 12.0,
            Some(Scancode::Down) => state.offset.1 -= 0.1,
            Some(Scancode::Left) => state.offset.0 -= 0.1,
            Some(Scancode::Right) => state.offset.0 += 0.1,
            Some(Scancode::Up) => state.offset.1 += 0.1,
            _ => debug!("Unhandled key {:?}", scancode),
        },
        Event::MouseButtonDown {
            x, y, mouse_btn, ..
        } => if mouse_btn == MouseButton::Left {
            if let Some((x, y)) = transform_coords(x, y, display.window().size(), &*state) {
                state.drag = SquarePart::from_coords(x, y);
            }
        },
        Event::MouseButtonUp { mouse_btn, .. } if mouse_btn == MouseButton::Left => {
            state.drag = None;
        }
        Event::MouseMotion { xrel, yrel, .. } => {
            let (x, y) = display.window().size();
            let xrel = xrel as f32 / x as f32;
            let yrel = yrel as f32 / y as f32;
            if let Some((part, angle)) = state.drag {
                match part {
                    SquarePart::Corner => unimplemented!(),
                    SquarePart::Edge => unimplemented!(),
                    SquarePart::Middle => unimplemented!(),
                }
            }
        }
        _ => debug!("Unhandled event {:?}", ev),
    }
    Ok(())
}

/// Transforms screen coordinates to coordinates on the square, with an OpenGL-like coordinate
/// space (i.e. (0, 0) at center, (1, 1) at top right). If the coordinates are invalid or don't
/// refer to a point on the square, returns None.
fn transform_coords(x: i32, y: i32, max: (u32, u32), state: &State) -> Option<(f32, f32)> {
    let x = x as f32;
    let y = y as f32;
    let max_x = max.0 as f32;
    let max_y = max.1 as f32;

    // Convert x and y to screen coordinates.
    let x = (2.0 * x / max_x) - 1.0;
    let y = 1.0 - (2.0 * y / max_y);

    //let xy_ = RowVector3::from_iterator([x, y, 1.0].into_iter().cloned()) * state.proj_inv;
    let xy_ = RowVector3::from_iterator([x, y, 1.0].into_iter().cloned());
    let x = xy_[0];
    let y = xy_[1];
    // xy_[2] should be the size of the square, I think?

    if -1.0 <= x && x <= 1.0 && -1.0 <= y && y <= 1.0 {
        Some((x, y))
    } else {
        None
    }
}
