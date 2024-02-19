use tracing::debug;

use super::voxel_state::VoxelRenderer;

pub trait DrawChunk<'a> {
    fn draw_chunks(
        &mut self,
        renderer: &'a VoxelRenderer,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
}
impl<'a, 'b> DrawChunk<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_chunks(
        &mut self,
        renderer: &'a VoxelRenderer,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
    ) {
        debug!("Rendering Chunks {:#?}", renderer.rendered_chunks.len());
        for chunk in &renderer.rendered_chunks {
            
            self.set_vertex_buffer(0, chunk.vertices.slice(..));
            self.set_index_buffer(chunk.indices.slice(..), wgpu::IndexFormat::Uint32);
            self.set_bind_group(0, &renderer.atlas_bind_group, &[]);
            self.set_bind_group(1, camera_bind_group, &[]);
            self.set_bind_group(2, light_bind_group, &[]);
            self.draw_indexed(0..chunk.number_of_indices, 0, 0..1);
        }
    }
}
