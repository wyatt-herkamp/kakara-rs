use std::{borrow::Cow, ops::Index};

use ahash::HashMap;

use crate::engine::voxel::Face;

use super::{TextureAtlas, UVCoordinates};

#[derive(Debug)]
pub enum CubeTextures<'a> {
    SingleTexture(Cow<'a, str>),
    MultiTexture(HashMap<Face, Cow<'a, str>>),
    Transparent,
}
impl CubeTextures<'_> {
    pub fn get_coordinates(&self, texture_atlas: &TextureAtlas) -> Option<LoadedCubeTexture> {
        match self {
            CubeTextures::SingleTexture(texture) => {
                let uv = texture_atlas.as_ref().get_uv_for_texture(texture)?;
                Some(LoadedCubeTexture::SingleTexture(uv))
            }
            CubeTextures::MultiTexture(textures) => {
                if textures.len() == 6 {
                    let front = texture_atlas
                        .as_ref()
                        .get_uv_for_texture(textures.get(&Face::North)?)?;
                    let back = texture_atlas
                        .as_ref()
                        .get_uv_for_texture(textures.get(&Face::South)?)?;
                    let top = texture_atlas
                        .as_ref()
                        .get_uv_for_texture(textures.get(&Face::Top)?)?;
                    let bottom = texture_atlas
                        .as_ref()
                        .get_uv_for_texture(textures.get(&Face::Bottom)?)?;
                    let left = texture_atlas
                        .as_ref()
                        .get_uv_for_texture(textures.get(&Face::West)?)?;
                    let right = texture_atlas
                        .as_ref()
                        .get_uv_for_texture(textures.get(&Face::East)?)?;

                    Some(LoadedCubeTexture::MultiTextureAllSet {
                        front,
                        back,
                        top,
                        bottom,
                        left,
                        right,
                    })
                } else {
                    todo!("Handle this case")
                }
            }
            CubeTextures::Transparent => None,
        }
    }
}

impl<'a> From<&'a str> for CubeTextures<'a> {
    fn from(texture: &'a str) -> Self {
        CubeTextures::SingleTexture(Cow::Borrowed(texture))
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum LoadedCubeTexture {
    SingleTexture(UVCoordinates),
    MultiTexture {
        textures: HashMap<Face, UVCoordinates>,
        default_texture: UVCoordinates,
    },
    MultiTextureAllSet {
        front: UVCoordinates,
        back: UVCoordinates,
        top: UVCoordinates,
        bottom: UVCoordinates,
        left: UVCoordinates,
        right: UVCoordinates,
    },
}
impl Index<Face> for LoadedCubeTexture {
    type Output = UVCoordinates;

    fn index(&self, index: Face) -> &Self::Output {
        match self {
            LoadedCubeTexture::SingleTexture(texture) => texture,
            LoadedCubeTexture::MultiTexture {
                textures,
                default_texture,
            } => textures.get(&index).unwrap_or(default_texture),
            LoadedCubeTexture::MultiTextureAllSet {
                front,
                back,
                top,
                bottom,
                left,
                right,
            } => match index {
                Face::North => front,
                Face::South => back,
                Face::Top => top,
                Face::Bottom => bottom,
                Face::West => left,
                Face::East => right,
            },
        }
    }
}
