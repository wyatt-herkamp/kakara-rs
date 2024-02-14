use super::texture::Texture;
use bytemuck::{Pod, Zeroable};
use std::{borrow::Cow, fmt::Debug};
pub mod obj_model;
pub mod renderer;

pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct BasicModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex for BasicModelVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<BasicModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
// lib.rs
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    pub _padding: u32,
    pub color: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    pub _padding2: u32,
}
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawInstance {
    model: [[f32; 4]; 4],
    normal: [[f32; 3]; 3],
}
impl Vertex for RawInstance {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<RawInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 19]>() as wgpu::BufferAddress,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct BlockInstance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}
// NEW!
impl BlockInstance {
    pub fn to_raw(&self) -> RawInstance {
        RawInstance {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
            normal: cgmath::Matrix3::from(self.rotation).into(),
        }
    }
}

impl From<BlockInstance> for RawInstance {
    fn from(instance: BlockInstance) -> Self {
        instance.to_raw()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoMaterialId;
pub trait Model: Debug {
    type Material: Material;
    type Mesh: Mesh<MaterialId = Self::MaterialId>;

    type MaterialId: Copy;

    fn new() -> Self;

    fn name(&self) -> Cow<'_, str>;

    fn meshes_iter(&self) -> impl Iterator<Item = &Self::Mesh>;

    fn get_material(&self, id: Self::MaterialId) -> Option<&Self::Material>;

    fn materials_iter(&self) -> impl Iterator<Item = &Self::Material>;
}
pub trait Mesh: Debug {
    type MaterialId: Copy;

    /// Returns the vertex buffer for the mesh
    fn get_vertex_buffer(&self) -> &wgpu::Buffer;
    fn get_index_buffer(&self) -> &wgpu::Buffer;

    fn get_num_elements(&self) -> u32;

    fn name(&self) -> Cow<'_, str>;

    fn material_id(&self) -> Self::MaterialId;
}

pub trait Material: Debug {
    fn get_texture(&self) -> &Texture;

    fn get_bind_group(&self) -> &wgpu::BindGroup;

    fn name(&self) -> Cow<'_, str>;
}
