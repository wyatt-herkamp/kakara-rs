use std::ops::{Index, IndexMut};

use crate::engine::voxel::LoadedCubeTexture;

use super::{complex_shape::SizedComplexShape, Quad, Shape, Vertex};
use glam::{Affine3A, Vec3};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIs, EnumIter, EnumString};
/// Faces of a Cube
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    EnumIter,
    EnumIs,
    EnumString,
    Display,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum Face {
    /// Face pointing towards the negative Z axis
    North = 0,
    /// Face pointing towards the positive Y axis
    Top = 1,
    /// Face pointing towards the negative Y axis
    Bottom = 2,
    /// Face pointing towards the negative X axis
    West = 3,
    /// Face pointing towards the positive X axis
    East = 4,
    /// Face pointing towards the positive Z axis
    South = 5,
}
impl From<Face> for usize {
    #[inline]
    fn from(face: Face) -> usize {
        face as u8 as usize
    }
}
/// Cube is a Complex Shape composed of 6 Quads
/// Quads are ordered as follows:
/// 0: North
/// 1: Top
/// 2: Bottom
/// 3: West
/// 4: East
/// 5: South
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cube {
    pub shape: SizedComplexShape<Quad, 6>,
}
impl From<[Quad; 6]> for Cube {
    fn from(shape: [Quad; 6]) -> Self {
        Cube {
            shape: SizedComplexShape::new(shape),
        }
    }
}
impl Index<Face> for Cube {
    type Output = Quad;

    fn index(&self, index: Face) -> &Self::Output {
        &self.shape[Into::<usize>::into(index)]
    }
}
impl IndexMut<Face> for Cube {
    fn index_mut(&mut self, index: Face) -> &mut Self::Output {
        &mut self.shape[Into::<usize>::into(index)]
    }
}
impl Cube {
    pub fn new(
        center: Vec3,
        size: Vec3,
        loaded_cube_texture: LoadedCubeTexture,
        model_transform: Option<Affine3A>,
    ) -> Self {
        let small = 0f32; //1.0 / 256.0;

        // Generate the quads for the cube. Y is up. Z is front. X is left.
        let x_left = center.x - size.x / 2.0;
        let x_right = center.x + size.x / 2.0;

        let y_up = center.y + size.y / 2.0;
        let y_down = center.y - size.y / 2.0;

        let z_front = center.z - size.z / 2.0;
        let z_back = center.z + size.z / 2.0;

        // Front is facing towards the negative Z axis (South).
        let south_cube = Quad::new_with_normal(
            Vec3::new(x_right, y_up, z_front + small),
            Vec3::new(x_left, y_up, z_front + small),
            Vec3::new(x_right, y_down, z_front + small),
            Vec3::new(x_left, y_down, z_front + small),
            loaded_cube_texture[Face::South],
            [0.0, 0.0, -1.0].into(),
        );

        // Back is facing towards the positive Z axis (North).
        let back_quad = Quad::new_with_normal(
            Vec3::new(x_left, y_up, z_back - small),
            Vec3::new(x_right, y_up, z_back - small),
            Vec3::new(x_left, y_down, z_back - small),
            Vec3::new(x_right, y_down, z_back - small),
            loaded_cube_texture[Face::North],
            [0.0, 0.0, 1.0].into(),
        );

        // Top is facing towards the positive Y axis (Up).
        let top_quad = Quad::new_with_normal(
            Vec3::new(x_right, y_up + small, z_back),
            Vec3::new(x_left, y_up + small, z_back),
            Vec3::new(x_right, y_up + small, z_front),
            Vec3::new(x_left, y_up + small, z_front),
            loaded_cube_texture[Face::Top],
            [0.0, 1.0, 0.0].into(),
        );

        // Bottom is facing towards the negative Y axis (Down).
        let bottom_quad = Quad::new_with_normal(
            Vec3::new(x_left, y_down - small, z_back),
            Vec3::new(x_right, y_down - small, z_back),
            Vec3::new(x_left, y_down - small, z_front),
            Vec3::new(x_right, y_down - small, z_front),
            loaded_cube_texture[Face::Bottom],
            [0.0, -1.0, 0.0].into(),
        );

        // Left is facing towards the negative X axis (West).
        let left_quad: Quad = Quad::new_with_normal(
            Vec3::new(x_left - small, y_up, z_front),
            Vec3::new(x_left - small, y_up, z_back),
            Vec3::new(x_left - small, y_down, z_front),
            Vec3::new(x_left - small, y_down, z_back),
            loaded_cube_texture[Face::West],
            [-1.0, 0.0, 0.0].into(),
        );

        // Right is facing towards the positive X axis (East).
        let right_quad = Quad::new_with_normal(
            Vec3::new(x_right + small, y_up, z_back),
            Vec3::new(x_right + small, y_up, z_front),
            Vec3::new(x_right + small, y_down, z_back),
            Vec3::new(x_right + small, y_down, z_front),
            loaded_cube_texture[Face::East],
            [1.0, 0.0, 0.0].into(),
        );
        let shape = SizedComplexShape::with_opt_transform(
            [
                south_cube,
                top_quad,
                bottom_quad,
                left_quad,
                right_quad,
                back_quad,
            ],
            model_transform,
        );
        Cube { shape }
    }
}

impl Shape for Cube {
    #[inline(always)]
    fn vertices(&self) -> Vec<Vertex> {
        self.shape.vertices()
    }
    #[inline(always)]
    fn number_of_verticies(&self) -> usize {
        24
    }
    #[inline(always)]
    fn push_verticies(&self, verticies: &mut Vec<Vertex>) {
        self.shape.push_verticies(verticies);
    }
    #[inline(always)]
    fn indices(&self) -> Vec<u32> {
        self.shape.indices()
    }
    #[inline(always)]
    fn number_of_indices(&self) -> usize {
        36
    }
    #[inline(always)]
    fn verticies_grouped(&self) -> Vec<[Vertex; 3]> {
        self.shape.verticies_grouped()
    }
}
