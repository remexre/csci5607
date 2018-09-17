use std::f32::consts::PI;

use common::{
    failure::Error,
    glium_sdl2::SDL2Facade,
    helpers::magnitude,
    nalgebra::{Matrix2, RowVector2},
    sdl2::{
        event::{Event, WindowEvent},
        keyboard::Scancode,
        mouse::MouseButton,
        Sdl,
    },
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
    Corner(f32),
    Edge(f32),
    Middle,
}

impl SquarePart {
    /// The width (in square coordinates, i.e. where the square is 2.0 units high and wide, and
    /// centered at (0.0, 0.0)).
    pub const EDGE_WIDTH: f32 = 0.5;

    /// Gets the part of the square corresponding to the given coordinates on the square.
    pub fn from_coords(x: f32, y: f32) -> Option<(SquarePart, f32)> {
        use self::AxisPart::*;
        use self::SquarePart::*;

        let x_edge = AxisPart::from_coord(x)?;
        let y_edge = AxisPart::from_coord(y)?;

        let part = match (x_edge, y_edge) {
            (Center, Center) => Middle,

            (Minus, Center) => Edge(PI),
            (Plus, Center) => Edge(0.0),
            (Center, Minus) => Edge(-PI / 2.0),
            (Center, Plus) => Edge(PI / 2.0),

            (Minus, Minus) => Corner(PI * 7.0 / 4.0),
            (Minus, Plus) => Corner(PI * 5.0 / 4.0),
            (Plus, Minus) => Corner(PI / 4.0),
            (Plus, Plus) => Corner(PI * 3.0 / 4.0),
        };
        let angle = y.atan2(x);
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
            Some(Scancode::C) => state.color = !state.color,
            Some(Scancode::L) => state.clear = !state.clear,
            Some(Scancode::Q) => state.running = false,
            Some(Scancode::R) => state.reset(),
            Some(Scancode::T) => state.triangle = !state.triangle,
            Some(Scancode::Comma) => state.brightness *= 0.9,
            Some(Scancode::Period) => state.brightness *= 1.1,
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
            let (x, y) = transform_coords(x, y, display.window().size(), &*state);
            if !(state.triangle && y < x) {
                state.drag = SquarePart::from_coords(x, y);
            }
        },
        Event::MouseButtonUp { mouse_btn, .. } if mouse_btn == MouseButton::Left => {
            state.drag = None;
        }
        Event::MouseMotion {
            x, y, xrel, yrel, ..
        } => {
            let (xmax, ymax) = display.window().size();
            let xrel = 2.0 * xrel as f32 / xmax as f32;
            let yrel = -2.0 * yrel as f32 / ymax as f32;
            let mag = magnitude(xrel, yrel);
            let mut angle = yrel.atan2(xrel) - state.rotation;

            if let Some((part, _)) = state.drag {
                match part {
                    SquarePart::Corner(corner_angle) => {
                        let factor = (corner_angle - angle).cos();
                        state.rotation += factor * mag;
                    }
                    SquarePart::Edge(edge_angle) => {
                        let factor = (edge_angle - angle).cos();
                        state.scale *= 1.0 + (factor * mag);
                    }
                    SquarePart::Middle => {
                        state.offset.0 += xrel;
                        state.offset.1 += yrel;
                    }
                }
            }
        }
        Event::Window {
            win_event: WindowEvent::Resized(w, h),
            ..
        } => {
            state.aspect_ratio = w as f32 / h as f32;
        }
        _ => debug!("Unhandled event {:?}", ev),
    }
    Ok(())
}

/// Transforms screen coordinates to coordinates on the square, with an OpenGL-like coordinate
/// space (i.e. (0, 0) at center, (1, 1) at top right). If the coordinates are invalid or don't
/// refer to a point on the square, returns None.
fn transform_coords(x: i32, y: i32, max: (u32, u32), state: &State) -> (f32, f32) {
    let x = x as f32;
    let y = y as f32;
    let max_x = max.0 as f32;
    let max_y = max.1 as f32;

    // Convert x and y to screen coordinates.
    let x = (2.0 * x / max_x) - 1.0;
    let y = 1.0 - (2.0 * y / max_y);

    // Untransform.
    let xy = RowVector2::from_iterator([x, y / state.aspect_ratio].into_iter().cloned());
    let rot = Matrix2::from_iterator(
        [
            state.rotation.cos(),
            state.rotation.sin(),
            -state.rotation.sin(),
            state.rotation.cos(),
        ].into_iter()
            .cloned(),
    );
    let off = RowVector2::from_iterator([state.offset.0, state.offset.1].into_iter().cloned());
    let xy = ((xy - off) / state.scale) * rot;
    (xy[0], xy[1])
}
