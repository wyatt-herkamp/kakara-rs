use std::{
    io,
    ops::Index,
    path::{Path, PathBuf},
    sync::Arc,
};

use ahash::{HashMap, HashMapExt};
use glam::{U64Vec2, Vec2};
use image::{DynamicImage, GenericImage, GenericImageView};
use thiserror::Error;
use tracing::{debug, warn};
pub mod cube_textures;
#[derive(Debug, Error)]
pub enum TextureAtlasBuildError {
    #[error("No file name provided")]
    NoFileName,
    #[error("Invalid image")]
    InvalidImage(#[from] image::ImageError),
    #[error("MINECRAFT_ASSETS not set, no textures to load")]
    NoTextures,
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),
}
#[derive(Debug, PartialEq)]
pub struct TextureAtlasBuilder {
    pub textures: Vec<RawImageReference>,
    pub max_width: u32,
    pub max_height: u32,
}

impl TextureAtlasBuilder {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            max_width: 0,
            max_height: 0,
        }
    }
    /// Loads textures from the minecraft assets folder
    /// Currently only supports 16x16 block textures
    #[tracing::instrument]
    pub fn load_from_minecraft_assets() -> Result<TextureAtlas, TextureAtlasBuildError> {
        let Ok(minecraft_assets_location) = std::env::var("MINECRAFT_ASSETS").map(PathBuf::from)
        else {
            warn!("MINECRAFT_ASSETS not set");
            return Err(TextureAtlasBuildError::NoTextures);
        };
        let blocks = minecraft_assets_location
            .join("minecraft")
            .join("textures")
            .join("block");

        let mut builder = TextureAtlasBuilder::new();
        for entry in std::fs::read_dir(blocks)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if path.extension().map(|v| v == "png").unwrap_or(false) {
                    let texture = RawImageReference::from_file(path)?;
                    if texture.dynamic_image.width() != 16 || texture.dynamic_image.height() != 16 {
                        continue;
                    }
                    builder.add_texture(texture);
                }
            }
        }
        builder.build()
    }
    pub fn add_texture(&mut self, texture: RawImageReference) {
        let (width, height) = texture.dynamic_image.dimensions();
        self.max_width = self.max_width.max(width);
        self.max_height = self.max_height.max(height);
        self.textures.push(texture);
    }
    /// Builds the texture atlas
    pub fn build(self) -> Result<TextureAtlas, TextureAtlasBuildError> {
        let number_of_textures = self.textures.len() as u32;
        let half = (number_of_textures as f32).sqrt().ceil() as u32;
        let mut textures = HashMap::with_capacity(self.textures.len());
        let image_width = half * self.max_width;

        let image_height = half * self.max_height;
        debug!(
            "Creating texture atlas with size: {}x{}",
            image_width, image_height
        );
        // Stitches the textures together
        let mut x = 0;
        let mut y = 0;
        let mut result_image = DynamicImage::new_rgba8(image_width, image_height);
        debug!("Stitching textures together");
        for raw_image in self.textures.iter() {
            let texture_ref = TextureRef {
                name: raw_image.name.clone(),
                width: raw_image.dynamic_image.width(),
                height: raw_image.dynamic_image.height(),
                x: x,
                y: y,
            };
            textures.insert(raw_image.name.clone(), texture_ref);
            debug!("Added texture: {} at X: {x} Y: {y}", raw_image.name);
            result_image.copy_from(&raw_image.dynamic_image, x, y)?;
            if x + self.max_width >= image_width {
                y += self.max_height;
                x = 0;
            } else {
                x += self.max_width;
            }
        }
        debug!("Created texture atlas with {} textures", textures.len());
        let textures = Arc::new(TextureAtlasInfo {
            textures,
            width: image_width,
            height: image_height,
            texture_width: self.max_width,
            texture_height: self.max_height,
        });
        Ok(TextureAtlas {
            info: textures,
            texture: result_image,
        })
    }
}
#[derive(Debug, PartialEq)]
pub struct RawImageReference {
    pub name: String,
    pub dynamic_image: DynamicImage,
}

impl RawImageReference {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, TextureAtlasBuildError> {
        let img = image::open(path.as_ref())?;
        let image_name = path
            .as_ref()
            .file_name()
            .and_then(|v| v.to_str().map(|v| v.to_string()))
            .ok_or(TextureAtlasBuildError::NoFileName)?;

        Ok(Self {
            name: image_name,
            dynamic_image: img,
        })
    }
}
#[derive(Debug, PartialEq)]
pub struct TextureAtlas {
    pub texture: DynamicImage,
    pub info: Arc<TextureAtlasInfo>,
}
impl AsRef<TextureAtlasInfo> for TextureAtlas {
    fn as_ref(&self) -> &TextureAtlasInfo {
        &self.info
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextureAtlasInfo {
    pub textures: HashMap<String, TextureRef>,
    pub width: u32,
    pub height: u32,
    pub texture_width: u32,
    pub texture_height: u32,
}
impl TextureAtlasInfo {
    pub fn get_texture(&self, name: &str) -> Option<&TextureRef> {
        self.textures.get(name)
    }
    /// Gets the UV coordinates for a texture
    /// 4 UV coordinates are returned in the following order:
    /// TOP LEFT, TOP RIGHT, BOTTOM LEFT, BOTTOM RIGHT
    pub fn get_uv_for_texture(&self, name: &str) -> Option<UVCoordinates> {
        // width: Is the width of the texture atlas
        // height: Is the height of the texture atlas
        // x: Is the x coordinate of the texture in the atlas
        // y: Is the y coordinate of the texture in the atlas
        // texture_width: is the size of every slot in the atlas
        // texture_height: is the size of every slot in the atlas

        let texture = self.textures.get(name)?;

        let x = texture.x as f32 / self.width as f32;
        let y = texture.y as f32 / self.height as f32;
        let width = texture.width as f32 / self.width as f32;
        let height = texture.height as f32 / self.height as f32;

        Some(UVCoordinates {
            top_left: Vec2::new(x, y),
            top_right: Vec2::new(x + width, y),
            bottom_left: Vec2::new(x, y + height),
            bottom_right: Vec2::new(x + width, y + height),
        })
    }
}
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct UVCoordinates {
    pub top_left: Vec2,
    pub top_right: Vec2,
    pub bottom_left: Vec2,
    pub bottom_right: Vec2,
}

impl Index<usize> for UVCoordinates {
    type Output = Vec2;

    fn index(&self, index: usize) -> &Self::Output {
        let index = if index > 3 { index % 4 } else { index };
        match index {
            0 => &self.top_left,
            1 => &self.top_right,
            2 => &self.bottom_left,
            3 => &self.bottom_right,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl UVCoordinates {
    pub fn iter(&self) -> impl Iterator<Item = Vec2> + '_ {
        vec![
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        ]
        .into_iter()
    }
}
impl IntoIterator for UVCoordinates {
    type Item = Vec2;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        ]
        .into_iter()
    }
}
impl TextureAtlas {
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use std::io;
    ///
    /// let texture_atlas = TextureAtlas::new();
    /// let path = Path::new("texture_atlas.png");
    ///
    /// match texture_atlas.save(path) {
    ///     Ok(()) => println!("Texture atlas saved successfully."),
    ///     Err(e) => eprintln!("Failed to save texture atlas: {}", e),
    /// }
    /// ```
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), io::Error> {
        self.texture
            .save(path)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextureRef {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub x: u32,
    pub y: u32,
}
impl TextureRef {
    pub fn coordinates(&self) -> U64Vec2 {
        U64Vec2::new(self.x as u64, self.y as u64)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use tracing::info;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    pub use super::*;

    #[test]
    pub fn test_from_minecraft_assets() -> anyhow::Result<()> {
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env())
            .init();
        let Ok(minecraft_assets_location) = std::env::var("MINECRAFT_ASSETS").map(PathBuf::from)
        else {
            println!("MINECRAFT_ASSETS not set, skipping test");
            return Ok(());
        };
        let blocks = minecraft_assets_location
            .join("minecraft")
            .join("textures")
            .join("block");

        let mut builder = TextureAtlasBuilder::new();
        for entry in std::fs::read_dir(blocks)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if path.extension().map(|v| v == "png").unwrap_or(false) {
                    let texture = RawImageReference::from_file(path)?;
                    if texture.dynamic_image.width() != 16 || texture.dynamic_image.height() != 16 {
                        continue;
                    }
                    builder.add_texture(texture);
                }
            }
        }
        let now = std::time::Instant::now();
        let atlas = builder.build()?;
        info!("Built texture atlas in {:?}", now.elapsed());

        info!("Time to save texture atlas to disk. This may take a while.");
        atlas.save("atlas.png")?;
        Ok(())
    }
}
