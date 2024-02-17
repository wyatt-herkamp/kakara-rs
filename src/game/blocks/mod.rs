use std::sync::{Arc};

use ahash::{HashMap, HashMapExt};

use crate::engine::voxel::{
    texture_atlas::{TextureAtlas, TextureAtlasInfo},
    CubeTextures, LoadedCubeTexture,
};
#[derive(Debug)]
pub struct Block {
    id: usize,
    name: &'static str,
    textures: Option<CubeTextures<'static>>,
}
#[derive(Debug)]
pub struct BlockRegistery {
    pub index: usize,
    pub blocks: HashMap<String, Block>,
    pub textures: HashMap<usize, LoadedCubeTexture>,
    pub texture_atlas_info: Arc<TextureAtlasInfo>,
}
impl BlockRegistery {
    pub fn new(texture_atlas_info: Arc<TextureAtlasInfo>) -> Self {
        Self {
            index: 0,
            blocks: HashMap::new(),
            textures: HashMap::new(),
            texture_atlas_info,
        }
    }
    pub fn get_texture_for_id(&self, id: usize) -> Option<LoadedCubeTexture> {
        self.textures.get(&id).cloned()
    }
    pub fn register_block(&mut self, name: &'static str, textures: Option<CubeTextures<'static>>) {
        self.blocks.insert(
            name.to_owned(),
            Block {
                id: self.index,
                name,
                textures,
            },
        );
        self.index += 1;
    }
    pub fn load_textures(&mut self, texture_atlas: &TextureAtlas) {
        for (_, block) in &self.blocks {
            let Some(textures) = block.textures.as_ref() else {
                continue;
            };
            let texture = textures.get_coordinates(texture_atlas);
            if let Some(texture) = texture {
                self.textures.insert(block.id, texture);
            }
        }
    }
}
pub fn create_block_registery(atlas: Arc<TextureAtlasInfo>) -> BlockRegistery {
    let mut registery = BlockRegistery::new(atlas);
    registery.register_block("air", None);
    registery.register_block("dirt", Some("dirt.png".into()));
    registery
}
