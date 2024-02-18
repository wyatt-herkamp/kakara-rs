use glam::Vec3;

use crate::engine::voxel::UVCoordinates;

use super::{Shape, Vertex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad {
    pub top_left: Vertex,
    pub top_right: Vertex,
    pub bottom_left: Vertex,
    pub bottom_right: Vertex,
}
impl Quad {
    pub fn new(
        top_left: Vertex,
        top_right: Vertex,
        bottom_left: Vertex,
        bottom_right: Vertex,
    ) -> Self {
        Quad {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }
    pub fn new_with_normal(
        top_left: Vec3,
        top_right: Vec3,
        bottom_left: Vec3,
        bottom_right: Vec3,
        uv_coordinates: UVCoordinates,
        normal: Vec3,
    ) -> Self {
        Quad {
            top_left: Vertex::new(top_left, uv_coordinates.top_left, normal),
            top_right: Vertex::new(top_right, uv_coordinates.top_right, normal),
            bottom_left: Vertex::new(bottom_left, uv_coordinates.bottom_left, normal),
            bottom_right: Vertex::new(bottom_right, uv_coordinates.bottom_right, normal),
        }
    }
}
impl Shape for Quad {
    fn vertices(&self) -> Vec<Vertex> {
        vec![
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        ]
    }
    fn push_verticies(&self, verticies: &mut Vec<Vertex>) {
        verticies.extend(self.vertices());
    }
    fn indices(&self) -> Vec<u32> {
        vec![2, 0, 3, 0, 1, 3]
    }
    fn push_indices(&self, indices: &mut Vec<u32>) {
        let offset = indices.len() as u32;
        indices.extend(self.indices().iter().map(|index| index + offset));
    }
    fn verticies_grouped(&self) -> Vec<[Vertex; 3]> {
        vec![
            [self.top_left, self.bottom_left, self.top_right],
            [self.top_right, self.bottom_left, self.bottom_right],
        ]
    }
}
