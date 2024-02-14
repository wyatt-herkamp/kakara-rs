use std::{borrow::Cow, iter, path::Path};

use tracing::debug;
use wgpu::util::DeviceExt as _;

use self::{
    cube_data::CUBE_INDICES,
    texture_atlas::{TextureAtlas, UVCoordinates},
};

use super::{
    render_types::{BasicModelVertex, Material, Mesh, Model, NoMaterialId},
    texture::Texture,
};

pub mod cube_data;
pub mod texture_atlas;
pub mod voxel_pipeline;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Face {
    Front,
    Back,
    Top,
    Bottom,
    Left,
    Right,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RenderVoxel {
    pub translucent: bool,
    //TODO texture reference
    pub id: i32,
}

impl Default for RenderVoxel {
    fn default() -> Self {
        Self {
            translucent: false,
            id: 0,
        }
    }
}
#[derive(Debug)]
pub struct Cube {
    pub mesh: CubeMesh,
    pub material: CubeMaterial,
}
#[derive(Debug)]
pub struct CubeMaterial {
    pub name: String,
    pub texture_atlas: texture_atlas::TextureAtlas,
    pub texture: Texture,
    pub bind_group: wgpu::BindGroup,
}
impl Material for CubeMaterial {
    fn get_texture(&self) -> &super::texture::Texture {
        &self.texture
    }

    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }
}
#[derive(Debug)]

pub struct CubeMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}
impl Mesh for CubeMesh {
    fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    fn get_index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    fn get_num_elements(&self) -> u32 {
        self.num_elements
    }

    fn name(&self) -> std::borrow::Cow<'_, str> {
        Cow::Borrowed("Cube/Mesh")
    }

    type MaterialId = NoMaterialId;

    fn material_id(&self) -> Self::MaterialId {
        NoMaterialId
    }
}

impl Model for Cube {
    type Mesh = CubeMesh;

    type Material = CubeMaterial;

    fn new() -> Self {
        todo!()
    }

    fn name(&self) -> std::borrow::Cow<'_, str> {
        Cow::Borrowed("Cube/Model")
    }

    fn meshes_iter(&self) -> impl Iterator<Item = &Self::Mesh> {
        iter::once(&self.mesh)
    }

    fn get_material(&self, _: NoMaterialId) -> Option<&Self::Material> {
        Some(&self.material)
    }

    fn materials_iter(&self) -> impl Iterator<Item = &Self::Material> {
        iter::once(&self.material)
    }

    type MaterialId = NoMaterialId;
}
pub fn block_vertices(texture_locations: UVCoordinates) -> Vec<BasicModelVertex> {
    debug!("Texture locations: {:?}", texture_locations);
    cube_data::CUBE_NORMALS
        .iter()
        .enumerate()
        .zip(cube_data::CUBE_VERTICES.iter())
        .map(|((index, &normal), &vertex)| BasicModelVertex {
            position: vertex,
            tex_coords: texture_locations[index].to_array(),
            normal,
        })
        .collect()
}

pub fn add_block_model(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
    texture_atlas: TextureAtlas,
    texture: &str,
) -> anyhow::Result<Cube> {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Cube Vertex Buffer"),
        contents: bytemuck::cast_slice(&block_vertices(
            texture_atlas
                .get_uv_for_texture(texture)
                .expect("Texture not found"),
        )),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Cube Index Buffer"),
        contents: bytemuck::cast_slice(&cube_data::CUBE_INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });
    let cube_mesh = CubeMesh {
        vertex_buffer,
        index_buffer,
        num_elements: cube_data::CUBE_INDICES.len() as u32,
        material: 0,
    };
    let texture = super::texture::Texture::from_image(
        device,
        queue,
        &texture_atlas.texture,
        Some("Texture Atlas "),
        false,
    )?;
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            },
        ],
        label: Some("cube_bind_group"),
    });
    let cube_material = CubeMaterial {
        name: "Cube Material".to_string(),
        texture_atlas,
        texture,
        bind_group,
    };

    Ok(Cube {
        mesh: cube_mesh,
        material: cube_material,
    })
}
