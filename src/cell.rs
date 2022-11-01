use crate::{Vector3d, Vertex};

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    S0,// Death
    S1,// Life
    S2,// Test
}

static S0_COLOR: Vector3d = Vector3d {x: 0.0, y: 0.0, z: 0.0};
static S1_COLOR: Vector3d = Vector3d {x: 0.2, y: 1.0, z: 0.2};
static S2_COLOR: Vector3d = Vector3d {x: 1.0, y: 0.2, z: 0.2};

impl State {
    pub fn to_color(&self) -> Vector3d {
        match self {
            State::S0 => S0_COLOR,
            State::S1 => S1_COLOR,
            State::S2 => S2_COLOR,
        }
    }
}

pub static VERTEX1: Vertex = Vertex { position: [1.0, 1.0] };
pub static VERTEX2: Vertex = Vertex { position: [1.0, -1.0] };
pub static VERTEX3: Vertex = Vertex { position: [-1.0, 1.0] };
pub static VERTEX4: Vertex = Vertex { position: [-1.0, -1.0] };