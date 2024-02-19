use core::num;
use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    sync::Arc,
};

use glam::Vec3;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::{
    engine::voxel::{
        chunk_mesh::{Voxel, VoxelVisiblity},
        VoxelLocation,
    },
    game::Game,
};

use super::position::{BlockPosition, ChunkPosition};
// 16x16x256
#[derive(Debug)]
pub struct Chunk {
    pub position: ChunkPosition,
    pub sections: Vec<Section>,
}
impl Chunk {
    pub fn new(position: ChunkPosition) -> Self {
        let mut sections = Vec::with_capacity(16);
        for i in 0..16 {
            let mut section = Section::default();
            section.position = i as u8;
            sections.push(section);
        }
        Self { position, sections }
    }
    pub fn generate_test_chunk() -> Self {
        let mut chunk = Self::new(ChunkPosition::new(0, 0));
        for i in 0..16 {
            chunk.sections[i].block_store[0] = Block::Block(1);
        }
        chunk
    }
    pub fn set_block(&mut self, position: BlockPosition, block: u32) {
        let section = position.section();

        let section = &mut self.sections[section];
        section.dirty = true;
        let index = position.relative_block() as usize;
        section.block_store[index] = block.into();
    }

    pub(crate) fn generate(position: ChunkPosition, noise: &Perlin) -> Chunk {
        let mut chunk = Self::new(position);
        let mut rng = rand::thread_rng();

        for x in 0..16 {
            for z in 0..16 {
                let number = rng.gen_range(0..6u32);
                let height = (noise.get([x as f64 / 16.0, z as f64 / 16.0]) * 16.0) as i64 + 32;
                for y in 0..height {
                    let position = BlockPosition::new(x, y, z);

                    chunk.set_block(position, number.into());
                }
            }
        }
        chunk
    }
}
// 16x16x16
#[derive(Clone, PartialEq, Eq)]
pub struct Section {
    /// This is the same as the index in the chunk
    /// However it is stored here for easy access
    /// Using u8 as there are only 16 sections
    pub position: u8,
    pub dirty: bool,
    pub building: bool,
    pub block_store: BlockStore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    /// Points to a regular block in its natural state. No modifcations
    Block(u32),
    //Ref(u32)
}
impl From<u32> for Block {
    fn from(i: u32) -> Self {
        Self::Block(i)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockStore {
    pub blocks: Box<[Block; 16 * 16 * 16]>,
}
impl Index<usize> for BlockStore {
    type Output = Block;
    fn index(&self, index: usize) -> &Self::Output {
        &self.blocks[index]
    }
}
impl IndexMut<usize> for BlockStore {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.blocks[index]
    }
}
impl Default for BlockStore {
    fn default() -> Self {
        Self {
            blocks: Box::new([Block::Block(0); 16 * 16 * 16]),
        }
    }
}
impl Debug for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Section")
            .field("position", &self.position)
            .field("dirty", &self.dirty)
            .field("building", &self.building)
            .field("blocks", &self.number_of_non_air_blocks())
            .finish()
    }
}
impl Default for Section {
    fn default() -> Self {
        Self {
            position: 0,
            dirty: false,
            building: false,
            block_store: BlockStore::default(),
        }
    }
}
impl Section {
    /// Block Zero is always the air block. No matter how the game loads it is a forced air block
    pub fn number_of_non_air_blocks(&self) -> usize {
        self.block_store
            .blocks
            .iter()
            .filter(|b| !matches!(b, Block::Block(0)))
            .count()
    }
    pub fn contains_non_air_blocks(&self) -> bool {
        self.block_store
            .blocks
            .iter()
            .any(|b| !matches!(b, Block::Block(0)))
    }
    pub fn get_voxels(&self, game: Arc<Game>) -> Vec<Voxel> {
        let mut voxels = Vec::with_capacity(16 * 16 * 16);
        for (index, block) in self.block_store.blocks.iter().enumerate() {
            #[allow(irrefutable_let_patterns)] // This is just future proofing code
            if let Block::Block(id) = *block {
                let position: Vec3 = Vec3::from_index(index);
                if id == 0 {
                    voxels.push(Voxel::air(position));
                    continue;
                } else {
                    voxels.push(Voxel::new(
                        position,
                        VoxelVisiblity::Opaque,
                        game.block_registery.get_texture_for_id(id as usize),
                    ));
                }
            }
        }
        voxels
    }
}
