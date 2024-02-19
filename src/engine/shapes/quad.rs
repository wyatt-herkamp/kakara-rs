use glam::Vec3;

use crate::engine::voxel::UVCoordinates;

use super::{cube::Face, Shape, Vertex};

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
    /// Creates a new Quad from a face
    /// # Arguments
    /// * `face` - The face to create the quad from
    /// * `position` - The position of the quad as the center of the a cube
    /// * `uv_coordinates` - The UV coordinates of the quad
    pub fn new_quad_from_face(
        face: Face,
        center: Vec3,
        size: Vec3,
        uv_coordinates: UVCoordinates,
    ) -> Self {
        let small = 0f32; //1.0 / 256.0;

        // Generate the quads for the cube. Y is up. Z is front. X is left.
        let x_left = center.x - size.x / 2.0;
        let x_right = center.x + size.x / 2.0;

        let y_up = center.y + size.y / 2.0;
        let y_down = center.y - size.y / 2.0;

        let z_front = center.z - size.z / 2.0;
        let z_back = center.z + size.z / 2.0;
        match face {
            Face::North => Quad::new_with_normal(
                Vec3::new(x_left, y_up, z_back - small),
                Vec3::new(x_right, y_up, z_back - small),
                Vec3::new(x_left, y_down, z_back - small),
                Vec3::new(x_right, y_down, z_back - small),
                uv_coordinates,
                face.normal().into(),
            ),
            Face::Top => Quad::new_with_normal(
                Vec3::new(x_right, y_up + small, z_back),
                Vec3::new(x_left, y_up + small, z_back),
                Vec3::new(x_right, y_up + small, z_front),
                Vec3::new(x_left, y_up + small, z_front),
                uv_coordinates,
                face.normal().into(),
            ),
            Face::Bottom => Quad::new_with_normal(
                Vec3::new(x_left, y_down - small, z_back),
                Vec3::new(x_right, y_down - small, z_back),
                Vec3::new(x_left, y_down - small, z_front),
                Vec3::new(x_right, y_down - small, z_front),
                uv_coordinates,
                face.normal().into(),
            ),
            Face::West => Quad::new_with_normal(
                Vec3::new(x_left - small, y_up, z_front),
                Vec3::new(x_left - small, y_up, z_back),
                Vec3::new(x_left - small, y_down, z_front),
                Vec3::new(x_left - small, y_down, z_back),
                uv_coordinates,
                face.normal().into(),
            ),
            Face::East => Quad::new_with_normal(
                Vec3::new(x_right + small, y_up, z_back),
                Vec3::new(x_right + small, y_up, z_front),
                Vec3::new(x_right + small, y_down, z_back),
                Vec3::new(x_right + small, y_down, z_front),
                uv_coordinates,
                face.normal().into(),
            ),
            Face::South => Quad::new_with_normal(
                Vec3::new(x_right, y_up, z_front + small),
                Vec3::new(x_left, y_up, z_front + small),
                Vec3::new(x_right, y_down, z_front + small),
                Vec3::new(x_left, y_down, z_front + small),
                uv_coordinates,
                face.normal().into(),
            ),
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
