use std::{mem, ops::Add};

use bytemuck::{Pod, Zeroable};
use glam::{UVec3, Vec3};
use strum::{EnumIs, IntoEnumIterator};
use wgpu::{
    util::DeviceExt as _, vertex_attr_array, BufferAddress, VertexBufferLayout, VertexStepMode,
};

use crate::engine::{
    render_types::ShaderVertexType,
    voxel::{cube_data::UntexturedQuad, VoxelLocation},
};

use super::{Face, LoadedCubeTexture};
const CHUNK_HEIGHT: usize = 16;
const CHUNK_WIDTH: usize = 16;
const CHUNK_DEPTH: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIs)]
pub enum VoxelVisiblity {
    Empty,
    Translucent,
    #[default]
    Opaque,
}
/// A single Voxel in the world with what faces are visible
#[derive(Debug, Clone, PartialEq)]
pub struct RenderVoxel<'a> {
    pub voxel: &'a Voxel,
    pub faces: Vec<Face>,
}
/// A single Voxel in the world
/// This is the highest level of abstraction for a voxel
/// Goes from Game Level Type -> Voxel -> [RenderVoxel] -> [BlockVertex]
#[derive(Debug, Clone, PartialEq)]
pub struct Voxel {
    pub position: Vec3,
    pub visibility: VoxelVisiblity,
    /// None is transparent
    pub textures: Option<LoadedCubeTexture>,
    // TODO: Custom Models. Update the Face method to use it.
}
impl Voxel {
    pub fn air(position: Vec3) -> Self {
        Self {
            position,
            visibility: VoxelVisiblity::Empty,
            textures: None,
        }
    }
    pub fn new(
        position: Vec3,
        visibility: VoxelVisiblity,
        textures: Option<LoadedCubeTexture>,
    ) -> Self {
        if visibility.is_opaque() && textures.is_none() {
            todo!("Give Coords of missing texture")
        }
        Self {
            position,
            visibility,
            textures,
        }
    }
    /// Gets the vertices for the given Face. Returns an array of 4 [BlockVertex]
    ///
    /// # Note
    /// Should be checked that the face is visible
    pub fn face(&self, face: Face) -> [BlockVertex; 4] {
        if self.visibility.is_empty() {
            unimplemented!("Empty Voxel")
        }
        let uv = self.textures.as_ref().map(|t| t[face]).unwrap_or_default();
        let face = face.get_quad() + self.position;
        face.as_block_voxel_array(uv)
    }
}
/// A visible voxel face in the world
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct BlockVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}
impl Default for BlockVertex {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            normal: [0.0; 3],
            uv: [0.0; 2],
        }
    }
}
impl ShaderVertexType for BlockVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use wgpu::VertexAttribute;

        static ATTRIBUTES: [VertexAttribute; 3] = vertex_attr_array![
            0 => Float32x3,
            1 => Float32x3,
            2 => Float32x2,
        ];

        VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}
#[derive(Debug, Clone)]
pub struct RawChunkMesh {
    pub position: UVec3,
    pub vertices: Vec<BlockVertex>,
    pub indices: Vec<u32>,
}
impl RawChunkMesh {
    /// Clears the vertices and indices
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
    /// Builds a new chunk mesh from a list of voxels
    /// See the `rebuild` method for more info
    pub fn build(voxels: Vec<Voxel>) -> RawChunkMesh {
        let mut this = Self {
            position: UVec3::new(0, 0, 0),
            vertices: Vec::with_capacity(voxels.len() * 2),
            indices: Vec::with_capacity(voxels.len() * 3),
        };
        this.rebuild(voxels);
        this
    }
    /// Rebuilds the chunk mesh from a list of voxels
    /// This will remove all existing vertices and indices
    /// and rebuild them from the list of voxels
    /// A list of Voxels is 4096 elements long. So an XYZ coordinate can be converted to an index as long as it is within the range of 0-15
    /// For more info on Voxel Location see the [VoxelLocation] trait
    ///
    pub fn rebuild(&mut self, voxels: Vec<Voxel>) {
        assert_eq!(
            voxels.len(),
            CHUNK_HEIGHT * CHUNK_WIDTH * CHUNK_DEPTH,
            "Invalid Voxel Count"
        );
        self.clear();
        let mut render_voxels = Vec::with_capacity(voxels.len());
        for index in 0..voxels.len() {
            let coords: UVec3 = UVec3::from_index(index);
            let current_voxel = &voxels[index];

            let mut faces = Vec::with_capacity(6);
            // Iterate over each face and check if it is visible
            // Face is visible if the next voxel is not opaque
            for face in Face::iter() {
                let next_voxel = coords
                    .position_for_face(face)
                    .map(|c| &voxels[c.to_index()]);
                if let Some(next_voxel) = next_voxel {
                    // Opaque is visible
                    if !next_voxel.visibility.is_opaque() && current_voxel.visibility.is_opaque() {
                        println!("Face: {:?}  at {} is visible", face, coords);
                        faces.push(face);
                    }
                } else {
                    faces.push(face);
                }
            }
            if faces.len() > 0 && !current_voxel.visibility.is_empty() {
                render_voxels.push(RenderVoxel {
                    voxel: current_voxel,
                    faces: faces,
                });
            }
        }
        let mut vertex_index = 0;
        // Calculate the vertexes and indicies for each render voxel
        for (render_voxel) in render_voxels.iter() {
            for face in &render_voxel.faces {
                // TODO Support Custom Models
                let face = render_voxel.voxel.face(*face);
                self.vertices.extend_from_slice(&face);
                UntexturedQuad::push_indicies(&mut self.indices, vertex_index);
                vertex_index += 4;
            }
        }
    }
}
#[derive(Debug)]
pub struct ChunkMesh {
    pub position: UVec3,
    pub vertices: wgpu::Buffer,
    pub indices: wgpu::Buffer,
    pub number_of_indices: u32,
}
impl ChunkMesh {
    /// Converts a RawChunkMesh into a ChunkMesh this will allocate the memory on the GPU
    pub fn build_mesh(raw_mesh: RawChunkMesh, device: &wgpu::Device) -> ChunkMesh {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("voxel_chunk_vertices"),
            contents: bytemuck::cast_slice(&raw_mesh.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("voxel_chunk_indices"),
            contents: bytemuck::cast_slice(&raw_mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        ChunkMesh {
            position: raw_mesh.position,
            vertices: vertex_buffer,
            indices: index_buffer,
            number_of_indices: raw_mesh.indices.len() as u32,
        }
    }

    pub fn render<'pass>(
        &'pass self,
        render_pass: &'pass mut wgpu::RenderPass<'pass>,
        texture: &'pass crate::engine::texture::Texture,
        light_bind_group: &'pass wgpu::BindGroup,
        camera_bind_group: &'pass wgpu::BindGroup,
    ) {
        render_pass.set_vertex_buffer(0, self.vertices.slice(..));
        render_pass.set_index_buffer(self.indices.slice(..), wgpu::IndexFormat::Uint32);
        //render_pass.set_bind_group(0, &material.get_bind_group(), &[]);
        render_pass.set_bind_group(1, camera_bind_group, &[]);
        render_pass.set_bind_group(2, light_bind_group, &[]);
        render_pass.draw_indexed(0..self.number_of_indices, 0, 0..1);
    }
}
