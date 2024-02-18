/*!
# Shapes
Type Representation of shapes. Heavily inspired on NickAcPT's shapes from his Minecraft Skin Renderer. [NMSR](https://github.com/NickAcPT/nmsr-rs/tree/main/nmsr-3d-renderer/nmsr-rendering/src/low_level/primitives)

*/
use bytemuck::{Pod, Zeroable};
use glam::{Affine3A, Vec2, Vec3};

pub use complex_shape::{ComplexShape, SizedComplexShape};
pub use cube::Cube;
pub use quad::Quad;

use super::render_types::ShaderVertexType;

pub mod complex_shape;
pub mod cube;
pub mod quad;

/// Basic Shape trait
#[auto_impl::auto_impl(&, &mut, Box, Rc, Arc)]
pub trait Shape {
    /// verticies of the shape
    fn vertices(&self) -> Vec<Vertex>;

    fn number_of_verticies(&self) -> usize {
        self.vertices().len()
    }
    fn push_verticies(&self, verticies: &mut Vec<Vertex>) {
        verticies.extend(self.vertices());
    }
    /// indices of the shape
    fn indices(&self) -> Vec<u32>;
    fn number_of_indices(&self) -> usize {
        self.indices().len()
    }
    fn push_indices(&self, indices: &mut Vec<u32>) {
        indices.extend(self.indices());
    }
    fn verticies_grouped(&self) -> Vec<[Vertex; 3]>;
}

///
pub type TextureCoordinates = Vec2;
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct Vertex {
    /// The position of the vertex
    pub position: Vec3,
    /// The uv coordinates of the vertex
    pub uv: TextureCoordinates,
    pub normal: Vec3,
}

impl ShaderVertexType for Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use wgpu::{
            vertex_attr_array, BufferAddress, VertexAttribute, VertexBufferLayout, VertexStepMode,
        };

        static ATTRIBUTES: [VertexAttribute; 3] = vertex_attr_array![
            0 => Float32x3,
            1 => Float32x3,
            2 => Float32x2,
        ];

        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}
impl Vertex {
    pub fn new(position: Vec3, uv: TextureCoordinates, normal: Vec3) -> Self {
        Vertex {
            position,
            uv,
            normal,
        }
    }

    pub fn transform(&self, model_transform: Affine3A) -> Self {
        if model_transform == Affine3A::IDENTITY {
            return *self;
        }

        let normal = model_transform.transform_vector3(self.normal).normalize();

        Vertex {
            position: model_transform.transform_point3(self.position),
            uv: self.uv,
            normal,
        }
    }
}

macro_rules! enum_shape {
    ($($name:ident($shape:ty)),*) => {
        $(
            impl From<$shape> for DynamicShape {
                fn from(shape: $shape) -> Self {
                    DynamicShape::$name(shape)
                }
            }
        )*

        impl Shape for DynamicShape {
            fn vertices(&self) -> Vec<Vertex> {
                match self {
                    $(
                        DynamicShape::$name(shape) => shape.vertices(),
                    )*
                }
            }
            fn push_verticies(&self, verticies: &mut Vec<Vertex>) {
                match self {
                    $(
                        DynamicShape::$name(shape) => shape.push_verticies(verticies),
                    )*
                }
            }
            fn number_of_verticies(&self) -> usize {
                match self {
                    $(
                        DynamicShape::$name(shape) => shape.number_of_verticies(),
                    )*
                }
            }
            fn indices(&self) -> Vec<u32> {
                match self {
                    $(
                        DynamicShape::$name(shape) => shape.indices(),
                    )*
                }
            }
            fn number_of_indices(&self) -> usize {
                match self {
                    $(
                        DynamicShape::$name(shape) => shape.number_of_indices(),
                    )*
                }
            }
            fn verticies_grouped(&self) -> Vec<[Vertex; 3]> {
                match self {
                    $(
                        DynamicShape::$name(shape) => shape.verticies_grouped(),
                    )*
                }
            }
        }
    };
}
#[derive(Debug, Clone, PartialEq)]
pub enum DynamicShape {
    Cube(Cube),
    Quad(Quad),
    ComplexShape(ComplexShape<DynamicShape>),
}
enum_shape!(
    Cube(Cube),
    Quad(Quad),
    ComplexShape(ComplexShape<DynamicShape>)
);
