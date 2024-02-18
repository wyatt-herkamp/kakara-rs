use crate::engine::{
    create_render_pipeline, render_types::ShaderVertexType, texture::Texture, SubRenderer,
    WGPUStateBase,
};

use super::{
    chunk_mesh::{BlockVertex, ChunkMesh, RawChunkMesh},
    TextureAtlas,
};
use flume::{Receiver, Sender};
use glam::UVec3;
use wgpu::PipelineLayout;

#[derive(Debug)]
pub enum ChunkUpdates {
    /// Adds a chunk to the renderer
    Add(RawChunkMesh),
    /// Removes a chunk from the renderer
    Remove(UVec3),
    /// Updates a chunk in the renderer
    Update(RawChunkMesh),
}
pub struct VoxelRendererConfig {
    pub texture_atlas: TextureAtlas,
    pub receiver: Receiver<ChunkUpdates>,
}
pub struct VoxelRenderer {
    pub rendered_chunks: Vec<ChunkMesh>,
    /// Updates to the voxel pipeline
    pub receiver: Receiver<ChunkUpdates>,
    pub voxel_pipeline: wgpu::RenderPipeline,
    pub texture: Texture,
    pub atlas_bind_group: wgpu::BindGroup,
}
impl VoxelRenderer {
    pub fn new(
        config: VoxelRendererConfig,
        wgpu: &WGPUStateBase,
        camera_layout: &wgpu::BindGroupLayout,
        light_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let VoxelRendererConfig {
            texture_atlas,
            receiver,
        } = config;
        let rendered_chunks = Vec::with_capacity(32);
        let texture_bind_group_layout =
            wgpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                    label: Some("Texture Atlas Group Layout"),
                });

        let pipeline_layout = wgpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Voxel Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, camera_layout, light_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = {
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Normal Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/voxel.wgsl").into()),
            };
            create_render_pipeline(
                &wgpu.device,
                &pipeline_layout,
                wgpu.config.format,
                Some(Texture::DEPTH_FORMAT),
                &[BlockVertex::desc()],
                shader,
            )
        };
        let texture = Texture::from_image(
            &wgpu.device,
            &wgpu.queue,
            &texture_atlas.texture,
            Some("Texture Atlas BOI"),
            false,
        )
        .expect("Failed to create texture from image");

        let texture_bind_group = wgpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
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
        let renderer = VoxelRenderer {
            rendered_chunks,
            receiver,
            voxel_pipeline: render_pipeline,
            texture,
            atlas_bind_group: texture_bind_group,
        };
        renderer
    }
}

impl SubRenderer for VoxelRenderer {
    fn update(&mut self, base_state: &mut WGPUStateBase, dt: std::time::Duration) {
        for update in self.receiver.try_iter() {
            match update {
                ChunkUpdates::Add(chunk) => {
                    let mesh = ChunkMesh::build_mesh(chunk, &base_state.device);
                    self.rendered_chunks.push(mesh);
                }
                ChunkUpdates::Remove(location) => {
                    if let Some(index) = self
                        .rendered_chunks
                        .iter()
                        .position(|chunk| chunk.position == location)
                    {
                        self.rendered_chunks.remove(index);
                    }
                }
                ChunkUpdates::Update(chunk) => {
                    if let Some(index) = self
                        .rendered_chunks
                        .iter()
                        .position(|c| c.position == chunk.position)
                    {
                        let mesh = ChunkMesh::build_mesh(chunk, &base_state.device);
                        self.rendered_chunks[index] = mesh;
                    }
                }
            }
        }
    }

    fn debug_info(&self) -> Vec<(String, String)> {
        vec![(
            "Rendered Chunks".to_string(),
            self.rendered_chunks.len().to_string(),
        )]
    }
}
