use derive_more::From;
use glam::{I64Vec3, UVec3, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From)]
pub struct ChunkPosition {
    pub x: i64,
    pub z: i64,
}
impl ChunkPosition {
    pub fn new(x: i64, z: i64) -> Self {
        Self { x, z }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, From, Hash)]
pub struct BlockPosition {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
impl BlockPosition {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}
macro_rules! convert_position_type {
    (
        $from:ty => $to:ty as $as:ty
    ) => {
        impl From<$from> for $to {
            fn from(pos: $from) -> Self {
                Self::new(pos.x as $as, pos.y as $as, pos.z as $as)
            }
        }
    };
}
convert_position_type!(BlockPosition => UVec3 as u32);
convert_position_type!(BlockPosition => I64Vec3 as i64);
convert_position_type!(BlockPosition => Vec3 as f32);

impl BlockPosition {
    pub fn chunk(&self) -> ChunkPosition {
        ChunkPosition {
            x: self.x >> 4,
            z: self.z >> 4,
        }
    }
    pub fn section(&self) -> usize {
        (self.y as usize >> 4)
    }
    /// TODO: this could be broken. It was written by AI
    /// The relative_block should be a 0-4095 value for the array of blocks in a section
    pub fn relative_block(&self) -> usize {
        let x = (self.x & 0xF) as usize;
        let y = self.y as usize & 0xF;
        let z = (self.z & 0xF) as usize;
        let index = (y << 8) | (z << 4) | x;
        debug_assert!(index < 4096, "Index out of bounds: {}", index);
        index
    }
    pub fn from_relative_block(relative_block: usize) -> Self {
        let x = relative_block & 0xF;
        let y = (relative_block >> 8) & 0xF;
        let z = (relative_block >> 4) & 0xF;
        Self {
            x: x as i64,
            y: y as i64,
            z: z as i64,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RawPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl RawPosition {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn chunk(&self) -> ChunkPosition {
        ChunkPosition {
            x: self.x.floor() as i64 >> 4,
            z: self.z.floor() as i64 >> 4,
        }
    }
}
impl Into<[f64; 3]> for RawPosition {
    fn into(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

impl From<[f64; 3]> for RawPosition {
    fn from([x, y, z]: [f64; 3]) -> Self {
        Self { x, y, z }
    }
}

impl Into<(f64, f64, f64)> for RawPosition {
    fn into(self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }
}

impl From<(f64, f64, f64)> for RawPosition {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RawRotation {
    pub yaw: f32,
    pub pitch: f32,
}
impl RawRotation {
    pub fn new(yaw: f32, pitch: f32) -> Self {
        Self { yaw, pitch }
    }
}

impl Into<[f32; 2]> for RawRotation {
    fn into(self) -> [f32; 2] {
        [self.yaw, self.pitch]
    }
}

impl From<[f64; 2]> for RawRotation {
    fn from([yaw, pitch]: [f64; 2]) -> Self {
        Self {
            yaw: yaw as f32,
            pitch: pitch as f32,
        }
    }
}

impl Into<(f32, f32)> for RawRotation {
    fn into(self) -> (f32, f32) {
        (self.yaw, self.pitch)
    }
}

impl From<(f32, f32)> for RawRotation {
    fn from((yaw, pitch): (f32, f32)) -> Self {
        Self { yaw, pitch }
    }
}
