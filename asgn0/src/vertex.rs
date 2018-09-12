#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pos: [f32; 2],
    color: [f32; 3],
    uv: [f32; 2],
}

implement_vertex!(Vertex, pos, color, uv);

pub static MODEL: &[Vertex] = &[
    Vertex {
        pos: [-1.0, -1.0],
        color: [0.0, 1.0, 0.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0],
        color: [0.0, 1.0, 0.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        pos: [1.0, 1.0],
        color: [0.0, 1.0, 0.0],
        uv: [1.0, 1.0],
    },
    Vertex {
        pos: [1.0, 1.0],
        color: [0.0, 1.0, 0.0],
        uv: [1.0, 1.0],
    },
    Vertex {
        pos: [1.0, -1.0],
        color: [0.0, 1.0, 0.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        pos: [-1.0, -1.0],
        color: [0.0, 1.0, 0.0],
        uv: [0.0, 0.0],
    },
];
