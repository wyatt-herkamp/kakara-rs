use glam::{Affine3A, Vec3};
use strum::EnumIter;

use self::cube_data::UntexturedQuad;

pub use location::VoxelLocation;
pub use texture_atlas::{
    cube_textures::CubeTextures, cube_textures::LoadedCubeTexture, TextureAtlas, TextureAtlasInfo,
    UVCoordinates,
};
pub mod basic_cube;
pub mod chunk_mesh;
pub mod cube_data;
mod location;
pub mod texture_atlas;
pub mod voxel_pipeline;
pub mod voxel_state;

pub use super::shapes::cube::Face;
impl Face {
    pub fn get_quad(&self) -> UntexturedQuad {
        match self {
            Face::North => cube_data::NORTH_FACE,
            Face::South => cube_data::SOUTH_FACE,
            Face::Top => cube_data::TOP_FACE,
            Face::Bottom => cube_data::BOTTOM_FACE,
            Face::West => cube_data::WEST_FACE,
            Face::East => cube_data::EAST_FACE,
        }
    }
}
