use crate::engine::{SubRenderer, WGPUStateBase};

use super::chunk_mesh::{ChunkMesh, RawChunkMesh};
use flume::{Receiver, Sender};
use glam::{I64Vec3};

#[derive(Debug)]
pub enum ChunkUpdates {
    /// Adds a chunk to the renderer
    Add(RawChunkMesh),
    /// Removes a chunk from the renderer
    Remove(I64Vec3),
    /// Updates a chunk in the renderer
    Update(RawChunkMesh),
}
pub struct VoxelRenderer {
    pub rendered_chunks: Vec<ChunkMesh>,
    /// Updates to the voxel pipeline
    pub receiver: Receiver<RawChunkMesh>,
}
impl VoxelRenderer {
    pub fn new(wgpu: &mut WGPUStateBase) -> (Self, Sender<RawChunkMesh>) {
        let (sender, receiver) = flume::unbounded();
        let rendered_chunks = Vec::with_capacity(32);

        let renderer = VoxelRenderer {
            rendered_chunks,
            receiver,
        };

        (renderer, sender)
    }
}

impl SubRenderer for VoxelRenderer {
    fn update(&mut self, base_state: &mut WGPUStateBase, dt: std::time::Duration) {
        todo!()
    }

    fn debug_info(&self) -> Vec<(String, String)> {
        todo!()
    }

    fn render<'pass>(
        &'pass mut self,
        wgpu: &'pass mut WGPUStateBase,
        render_pass: &'pass mut wgpu::RenderPass,
    ) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}
