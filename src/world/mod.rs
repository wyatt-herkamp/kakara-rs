use ahash::HashMap;

use self::{chunk::Chunk, position::ChunkPosition};

pub mod position;
pub mod chunk;

pub struct World{
    pub chunks: HashMap<ChunkPosition, Chunk>
}

