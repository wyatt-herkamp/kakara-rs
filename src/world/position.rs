use derive_more::From;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChunkPosition {
    pub x: i64,
    pub z: i64,
}
#[derive(Debug, Clone, Copy, PartialEq, From)]
pub struct BlockPosition {
    pub x: i64,
    pub y: i8,
    pub z: i64,
}

impl BlockPosition {
    pub fn chunk(&self) -> ChunkPosition {
        ChunkPosition {
            x: self.x >> 4,
            z: self.z >> 4,
        }
    }
    pub fn section(&self) -> i8 {
        (self.y >> 4)
    }
    pub fn relative_block(&self) -> i32 {
        let x = (self.x & 0xF) as i32;
        let y = self.y as i32 & 0xF;
        let z = (self.z & 0xF) as i32;
        (y << 8) | (z << 4) | x
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RawPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RawRotation {
    pub yaw: f32,
    pub pitch: f32,
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
