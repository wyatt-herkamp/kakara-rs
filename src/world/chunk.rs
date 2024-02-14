use ahash::HashMap;

use super::position::ChunkPosition;
// 16x16x256
pub struct Chunk {
    position: ChunkPosition,
    sections: [Section; 16],
}
impl Chunk {
    pub fn new(position: ChunkPosition) -> Self {
        let mut sections = [Section::default(); 16];
        for i in 0..16 {
            sections[i].position = i as i8;
        }
        Self { position, sections }
    }
}
// 16x16x16
#[derive(Debug, Clone, Copy)]
pub struct Section {
    position: i8,
    dirty: bool,
    building: bool,
    blocks: [i32; 16 * 16 * 16],
}

impl Default for Section {
    fn default() -> Self {
        Self {
            position: 0,
            dirty: false,
            building: false,
            blocks: [0; 16 * 16 * 16],
        }
    }
}
