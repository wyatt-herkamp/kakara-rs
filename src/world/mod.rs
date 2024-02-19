use std::sync::Arc;

use ahash::{HashMap, HashMapExt};
use flume::Sender;
use noise::Perlin;
use tracing::{debug, info};

use crate::{
    engine::voxel::{chunk_mesh::RawChunkMesh, voxel_state::ChunkUpdates},
    game::{Game, Player},
};

use self::{
    chunk::Chunk,
    position::{BlockPosition, ChunkPosition},
};

pub mod chunk;
pub mod position;
#[derive(Debug)]
pub struct World {
    pub noise: Perlin,
    pub chunks: HashMap<ChunkPosition, Chunk>,
    // ChunkMeshes are 16x16x16 meaning they need 3 points to define a position
    // The Block posistion is the bottom left corner of the chunk
    // Meshes are 16x16x16 meaning that they have multiple meshes for each chunk
    pub built_meshes: HashMap<BlockPosition, RawChunkMesh>,
    pub meshes_being_rendered: HashMap<BlockPosition, RawChunkMesh>,
}

impl World {
    pub fn new() -> Self {
        let noise = Perlin::default();

        Self {
            noise,
            chunks: HashMap::new(),
            built_meshes: HashMap::new(),
            meshes_being_rendered: HashMap::new(),
        }
    }

    pub fn generate_test_world() -> Self {
        let mut world = Self::new();
        for x in -1..1 {
            for z in -1..1 {
                let position = ChunkPosition::new(x, z);
                world
                    .chunks
                    .insert(position, Chunk::generate(position, &world.noise));
            }
        }
        world
    }
    /// Updates the meshes that are being rendered
    /// Currently we are cloning the meshes to send them to the renderer.The renderer converts these raw meshes into the the wgpu Buffers
    pub fn update_meshes(
        &mut self,
        player: &Player,
        game: Arc<Game>,
        sender: Sender<ChunkUpdates>,
    ) {
        // Step One: Get the chunks that need to be rendered
        let player_chunk = player.position.chunk();
        let mut chunks_needing_rendering = Vec::with_capacity(
            (game.game_settings.chunk_render_distance * 2)
                * (game.game_settings.chunk_render_distance * 2),
        );
        let render_distance = game.game_settings.chunk_render_distance as i64;
        for x in -render_distance..render_distance {
            for z in -render_distance..render_distance {
                let position = ChunkPosition::new(x + player_chunk.x, z + player_chunk.z);
                if !self.chunks.contains_key(&position) {
                    // TODO: This will cause a lot of blocking so we should open a task and have a "callback"
                    self.chunks
                        .insert(position, Chunk::generate(position, &self.noise));
                }
                chunks_needing_rendering.push(position);
            }
        }
        // Step Two: Remove the meshes that are should not be rendered
        self.meshes_being_rendered
            .extract_if(|k, _| chunks_needing_rendering.contains(&k.chunk()))
            .for_each(|(location, mesh)| {
                sender
                    .send(ChunkUpdates::Remove(location.clone().into()))
                    .unwrap();
                self.built_meshes.insert(location, mesh);
            });
        // Step Three: Build Meshes the need to be rendered. Update the ones that are already being rendered
        for position in &chunks_needing_rendering {
            for y in 0usize..16 {
                // TODO no errors
                let chunk = self.chunks.get_mut(&position).expect("Chunk not found");
                let section = &mut chunk.sections[y];

                let section_position =
                    BlockPosition::new(position.x * 16, y as i64 * 16, position.z * 16);
                debug!(
                    "Chunk Position {:?} will be rendered with {} non air blocks",
                    section_position,
                    section.number_of_non_air_blocks()
                );
                if !section.contains_non_air_blocks() {
                    debug!("Skipping section because it has no non air blocks");
                    continue;
                }
                if let Some(mesh) = self.meshes_being_rendered.get_mut(&section_position) {
                    if section.dirty {
                        // Dirty section so rebuild the mesh
                        mesh.rebuild(section.get_voxels(game.clone()));
                        section.dirty = false;
                    }
                    sender.send(ChunkUpdates::Update(mesh.clone())).unwrap();
                } else if let Some(mut mesh) = self.built_meshes.remove(&section_position) {
                    // Already built so just add it to the being rendered
                    if section.dirty {
                        // Dirty section so rebuild the mesh
                        mesh.rebuild(section.get_voxels(game.clone()));
                        section.dirty = false;
                    }
                    self.meshes_being_rendered
                        .insert(section_position, mesh.clone());
                    sender.send(ChunkUpdates::Add(mesh)).unwrap();
                } else {
                    // Basically the mesh is not built already so add it.
                    let mesh = RawChunkMesh::build(
                        section_position.into(),
                        section.get_voxels(game.clone()),
                    );
                    section.dirty = false;
                    self.meshes_being_rendered
                        .insert(section_position, mesh.clone());
                    sender.send(ChunkUpdates::Add(mesh)).unwrap();
                }
            }
        }
        info!("Updated Meshes");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_generation() {
        let world = World::generate_test_world();
        println!("{:?}", world.chunks)
    }
}
