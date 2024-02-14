use crate::engine::render_types::BasicModelVertex;

use super::texture_atlas::UVCoordinates;

#[derive(Debug)]
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
}

#[rustfmt::skip]
pub static CUBE_INDICES: &[u32] = &[
    // Front
    0, 2, 1, 3, 1, 2,
    // back
    4, 5, 6, 6, 5, 7,
    // top
    10, 9, 8, 11, 9, 10,
    // bottom
    12, 13, 14, 14, 13, 15,
    // Left
    16, 18, 17, 16, 17, 18,
    // Right
    20, 21, 22, 22, 21, 23
];

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
        // Top Right
        [1f32, 1f32, -1f32],
        // Bottom Left
        [-1f32, -1f32, -1f32],
        // Bottom Right
        [1f32, -1f32, -1f32],
    ],
    normal: [[0f32, 0f32, -1f32]; 4],
};

pub static TOP_FACE: UntexturedQuad = UntexturedQuad {
    position: [
        [-1f32, 1f32, -1f32],
        [1f32, 1f32, -1f32],
        // Top
        [-1f32, 1f32, 1f32],
        [1f32, 1f32, 1f32],
    ],
    normal: [[0f32, 1f32, 0f32]; 4],
};

pub static BOTTOM_FACE: UntexturedQuad = UntexturedQuad {
    position: [
        [-1f32, -1f32, -1f32],
        [1f32, -1f32, -1f32],
        [-1f32, -1f32, 1f32],
        [1f32, -1f32, 1f32],
    ],
    normal: [[0f32, -1f32, 0f32]; 4],
};
pub static LEFT_FACE: UntexturedQuad = UntexturedQuad {
    position: [
        // Touches Front Bottom Left (Top Left)
        [-1f32, -1f32, 1f32],
        // Touches Front Top Left (Top Right)
        [-1f32, 1f32, 1f32],
        // Touches Back Top Left (Bottom Right)
        [-1f32, 1f32, -1f32],
        // Touches Back Bottom Left (Bottom Left)
        [-1f32, -1f32, -1f32],
    ],
    normal: [[-1f32, 0f32, 0f32]; 4],
};

pub static RIGHT_FACE: UntexturedQuad = UntexturedQuad {
    position: [
        [1f32, 1f32, 1f32],
        [1f32, -1f32, 1f32],
        [1f32, 1f32, -1f32],
        [1f32, -1f32, -1f32],
    ],
    normal: [[1f32, 0f32, 0f32]; 4],
};
