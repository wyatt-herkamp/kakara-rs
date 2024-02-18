use glam::{U16Vec3, U64Vec3, UVec3, Vec3};

use super::Face;

pub trait VoxelLocation {
    /// Converts a usize index to a VoxelLocation type
    /// Will be 0-4095
    fn from_index(index: usize) -> Self;
    /// Converts a VoxelLocation type to a usize index
    fn to_index(&self) -> usize;
    #[inline(always)]
    fn as_u32(&self) -> u32 {
        self.to_index() as u32
    }
    #[inline(always)]
    fn as_u64(&self) -> u64 {
        self.to_index() as u64
    }
    #[inline(always)]
    fn as_u16(&self) -> u16 {
        self.to_index() as u16
    }

    fn is_valid_position(&self) -> bool;
    /// Returns None if the face is out of bounds
    fn position_for_face(&self, face: Face) -> Option<Self>
    where
        Self: Sized;

    // TODO: Create a Voxelocation Iterator
}
macro_rules! impl_voxel_location {
    ($t:ty as $a:ty) => {
        impl VoxelLocation for $t {
            fn from_index(index: usize) -> Self {
                debug_assert!(index < 4096, "Index out of bounds: {}", index);
                let x = index & 0xF;
                let y = (index >> 8) & 0xF;
                let z = (index >> 4) & 0xF;
                Self::new(x as $a, y as $a, z as $a)
            }
            fn to_index(&self) -> usize {
                let x = (self.x as usize & 0xF);
                let y = self.y as usize & 0xF;
                let z = (self.z as usize & 0xF);
                let result = (y << 8) | (z << 4) | x;
                debug_assert!(result < 4096, "Index out of bounds: {}", result);
                return result;
            }
            fn is_valid_position(&self) -> bool {
                (self.x < 16 as $a && self.y < 16 as $a && self.z < 16 as $a)
                    && (self.x >= 0 as $a && self.y >= 0 as $a && self.z >= 0 as $a)
            }
            fn position_for_face(&self, face: Face) -> Option<Self>
            where
                Self: Sized,
            {
                let mut new = *self;
               
                match face {
                    Face::North => {
                        if new.z == 15 as $a {
                            return None;
                        }
                        new.z += 1 as $a;
                    }
                    Face::South => {
                        if new.z == 0 as $a {
                            return None;
                        }
                        new.z -= 1 as $a;
                    }
                    Face::West => {
                        if new.x == 0 as $a {
                            return None;
                        }
                        new.x -= 1 as $a;
                    }
                    Face::East => {
                        if new.x == 15 as $a {
                            return None;
                        }
                        new.x += 1 as $a;
                    }
                    Face::Top => {
                        if new.y == 15 as $a {
                            return None;
                        }
                        new.y += 1 as $a;
                    }
                    Face::Bottom => {
                        if new.y == 0 as $a {
                            return None;
                        }
                        new.y -= 1 as $a;
                    }
                }
                if new.is_valid_position() {
                    Some(new)
                } else {
                    None
                }
            }
        }
    };
}
impl_voxel_location!(UVec3 as u32);
impl_voxel_location!(U64Vec3 as u64);
impl_voxel_location!(U16Vec3 as u16);
impl_voxel_location!(Vec3 as f32);
