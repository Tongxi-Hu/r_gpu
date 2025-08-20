use wgpu::util::DeviceExt;

use crate::common::{Color, Normal, Position, Rotation, Scale, Vertex};

pub const POSITION: &[Position] = &[
    [-100.0, 100.0, 100.0],   //0
    [-100.0, -100.0, 100.0],  //1
    [100.0, 100.0, 100.0],    //2
    [100.0, -100.0, 100.0],   //3
    [100.0, 100.0, -100.0],   //4
    [100.0, -100.0, -100.0],  //5
    [-100.0, 100.0, -100.0],  //6
    [-100.0, -100.0, -100.0], //7
];

pub const COLOR: &[Color] = &[
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [1.0, 1.0, 0.0],
    [1.0, 0.0, 1.0],
    [0.0, 1.0, 1.0],
];

pub const NORMAL: &[Normal] = &[
    [0.0, 0.0, 1.0],
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [-1.0, 0.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, 0.0, -1.0],
];

pub const INDEX: &[u32] = &[
    0, 1, 2, 2, 1, 3, // front
    2, 3, 4, 4, 3, 5, // right
    0, 2, 4, 0, 4, 6, // top
    0, 7, 1, 0, 6, 7, // left
    1, 7, 5, 1, 5, 3, // bottom
    4, 5, 7, 4, 7, 6, //back
];

// position info
pub const DEFAULT_SCALE: Scale = [1.0, 1.0, 1.0];
pub const DEFAULT_ROTATION: Rotation = [45.0, 45.0, 0.0];
pub const DEFAULT_POSITION: Position = [0., 0.0, -1300.0];

pub fn generate_cube_vertex() -> Vec<Vertex> {
    let mut vertex = vec![];
    for (index, &v) in INDEX.iter().enumerate() {
        vertex.push(Vertex {
            position: POSITION[v as usize],
            color: COLOR[index / 6],
            normal: NORMAL[index / 6],
        });
    }
    vertex
}
