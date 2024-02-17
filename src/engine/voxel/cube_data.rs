/*!
# cube_data

Contains layout information for basic cubes.
*/
use std::ops::Add;

use glam::Vec3;

use crate::engine::render_types::BasicModelVertex;

use super::{chunk_mesh::BlockVertex, texture_atlas::UVCoordinates};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UntexturedQuad {
    pub position: [[f32; 3]; 4],
    pub normal: [[f32; 3]; 4],
}
impl UntexturedQuad {
    pub fn into_raw_vertex(self, uv_coords: UVCoordinates) -> Vec<BasicModelVertex> {
        let mut vertices = Vec::new();
        self.push_raw_vertecies(&mut vertices, uv_coords);
        vertices
    }
    pub fn push_raw_vertecies(
        &self,
        vertices: &mut Vec<BasicModelVertex>,
        uv_coords: UVCoordinates,
    ) {
        for i in 0..4 {
            vertices.push(BasicModelVertex {
                position: self.position[i],
                normal: self.normal[i],
                tex_coords: uv_coords[i].to_array(),
            });
        }
    }
    pub fn as_block_voxel_array(&self, uv: UVCoordinates) -> [BlockVertex; 4] {
        let UntexturedQuad { position, normal } = self;
        [
            BlockVertex {
                position: position[0],
                normal: normal[0],
                uv: uv[0].into(),
            },
            BlockVertex {
                position: position[1],
                normal: normal[1],
                uv: uv[1].into(),
            },
            BlockVertex {
                position: position[2],
                normal: normal[2],
                uv: uv[2].into(),
            },
            BlockVertex {
                position: position[3],
                normal: normal[3],
                uv: uv[3].into(),
            },
        ]
    }
    pub fn indicies(offset: u32) -> [u32; 6] {
        [
            0 + offset,
            2 + offset,
            1 + offset,
            3 + offset,
            1 + offset,
            2 + offset,
        ]
    }

    pub fn push_indicies(indicies: &mut Vec<u32>, offset: u32) {
        indicies.push(0 + offset);
        indicies.push(2 + offset);
        indicies.push(1 + offset);
        indicies.push(3 + offset);
        indicies.push(1 + offset);
        indicies.push(2 + offset);
    }
}

fn add_vec3_to_f32_array(array: &mut [f32; 3], vec: Vec3) {
    array[0] += vec.x;
    array[1] += vec.y;
    array[2] += vec.z;
}
impl Add<Vec3> for UntexturedQuad {
    type Output = UntexturedQuad;

    fn add(self, rhs: Vec3) -> Self::Output {
        let mut new_quad = self;
        for i in 0..4 {
            add_vec3_to_f32_array(&mut new_quad.position[i], rhs);
        }
        new_quad
    }
}
/// The base indicies for a quad
pub static BASE_INDICIES: [u32; 6] = [0, 2, 1, 3, 1, 2];
pub static FRONT_FACE: UntexturedQuad = UntexturedQuad {
    position: [
        // Top Left
        [-1f32, 1f32, 1f32],
        // Top Right
        [1f32, 1f32, 1f32],
        // Bottom Left
        [-1f32, -1f32, 1f32],
        // Bottom Right
        [1f32, -1f32, 1f32],
    ],
    normal: [[0f32, 0f32, 1f32]; 4],
};

pub static BACK_FACE: UntexturedQuad = UntexturedQuad {
    position: [
        // Top Left
        [-1f32, 1f32, -1f32],
        [-1f32, -1f32, -1f32],
        [1f32, -1f32, -1f32],
        [1f32, 1f32, -1f32],
    ],
    normal: [[0f32, 0f32, -1f32]; 4],
};

pub static TOP_FACE: UntexturedQuad = UntexturedQuad {
    position: [
        [-1f32, 1f32, -1f32],
        [-1f32, 1f32, 1f32],
        [1f32, 1f32, 1f32],
        [1f32, 1f32, -1f32],
    ],
    normal: [[0f32, 1f32, 0f32]; 4],
};

pub static BOTTOM_FACE: UntexturedQuad = UntexturedQuad {
    position: [
        [-1f32, -1f32, -1f32],
        [-1f32, -1f32, 1f32],
        [1f32, -1f32, 1f32],
        [1f32, -1f32, -1f32],
    ],
    normal: [[0f32, -1f32, 0f32]; 4],
};
pub static WEST_FACE: UntexturedQuad = UntexturedQuad {
    position: [
        [-1f32, 1f32, -1f32],
        [-1f32, -1f32, -1f32],
        [-1f32, -1f32, 1f32],
        [-1f32, 1f32, 1f32],
    ],
    normal: [[-1f32, 0f32, 0f32]; 4],
};

pub static EAST_FACE: UntexturedQuad = UntexturedQuad {
    position: [
        [-1f32, 1f32, -1f32],
        [1f32, -1f32, 1f32],
        [1f32, -1f32, -1f32],
        [1f32, 1f32, -1f32],
    ],
    normal: [[1f32, 0f32, 0f32]; 4],
};
