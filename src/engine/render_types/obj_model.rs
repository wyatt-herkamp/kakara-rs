use crate::engine::{
    render_types::BasicModelVertex,
    texture::{Texture, TextureError},
};
use std::{
    borrow::Cow,
    io::{BufReader, Cursor},
};

use thiserror::Error;
use tracing::error;
use wgpu::util::DeviceExt;

use super::{Material, Mesh, Model};
#[derive(Debug, Error)]
pub enum OBJModelError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Tobj Error: {0}")]
    TobjError(#[from] tobj::LoadError),
    #[error("Wgpu Error: {0}")]
    WgpuError(#[from] wgpu::RequestDeviceError),
    #[error("Texture Error: {0}")]
    TextureError(#[from] TextureError),
    #[error("Missing Material")]
    MissingMaterial,
}
type Result<T> = std::result::Result<T, OBJModelError>;
#[derive(Debug)]
pub struct OBJMaterial {
    pub name: String,
    pub diffuse_texture: Texture,
    pub bind_group: wgpu::BindGroup,
}
impl Material for OBJMaterial {
    fn get_texture(&self) -> &Texture {
        &self.diffuse_texture
    }

    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    fn name(&self) -> std::borrow::Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }
}
impl OBJMaterial {
    pub fn new(
        device: &wgpu::Device,
        name: &str,
        diffuse_texture: Texture,
        layout: &wgpu::BindGroupLayout,
    ) -> Self {
        println!("Creating material: {}", name);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some(name),
        });

        Self {
            name: String::from(name),
            diffuse_texture,
            bind_group,
        }
    }
}

#[derive(Debug)]
pub struct OBJMesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}
impl Mesh for OBJMesh {
    type MaterialId = usize;

    fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    fn get_index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }
    fn get_num_elements(&self) -> u32 {
        self.num_elements
    }

    fn name(&self) -> std::borrow::Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }

    fn material_id(&self) -> usize {
        self.material
    }
}
#[derive(Debug)]
pub struct OBJModel {
    pub meshes: Vec<OBJMesh>,
    pub materials: Vec<OBJMaterial>,
}
impl Model for OBJModel {
    type Mesh = OBJMesh;
    type Material = OBJMaterial;
    type MaterialId = usize;

    fn new() -> Self {
        Self {
            meshes: Vec::new(),
            materials: Vec::new(),
        }
    }

    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("OBJ Model")
    }

    fn meshes_iter(&self) -> impl Iterator<Item = &Self::Mesh> {
        self.meshes.iter()
    }

    fn materials_iter(&self) -> impl Iterator<Item = &Self::Material> {
        self.materials.iter()
    }

    fn get_material(&self, id: Self::MaterialId) -> Option<&Self::Material> {
        self.materials.get(id)
    }
}

pub async fn load_string(file_name: &str) -> Result<String> {
    let path = std::path::Path::new(file_name);
    let txt = std::fs::read_to_string(path)?;
    Ok(txt)
}

pub async fn load_binary(file_name: &str) -> Result<Vec<u8>> {
    let path = std::path::Path::new(file_name);

    let data = std::fs::read(path)?;

    Ok(data)
}

pub async fn load_texture(
    file_name: &str,
    is_normal_map: bool,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Result<Texture> {
    let data = load_binary(file_name).await?;
    Texture::from_bytes(device, queue, &data, file_name, is_normal_map).map_err(Into::into)
}

pub async fn load_model(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
) -> Result<OBJModel> {
    let obj_text = load_string(file_name).await?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    let (models, obj_materials) = tobj::load_obj_buf_async(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move {
            let mat_text = load_string(&p).await.unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )
    .await?;
    let mut materials = Vec::new();
    for m in obj_materials? {
        let diffuse_texture_location = m.diffuse_texture.ok_or_else(|| {
            error!("Material {} is missing a diffuse texture", m.name);
            OBJModelError::MissingMaterial
        })?;
        let diffuse_texture = load_texture(&diffuse_texture_location, false, device, queue).await?;

        materials.push(OBJMaterial::new(device, &m.name, diffuse_texture, layout));
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| BasicModelVertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                    normal: [
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2],
                    ],
                })
                .collect::<Vec<_>>();

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{} Vertex Buffer", file_name)),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{} Index Buffer", file_name)),
                contents: bytemuck::cast_slice(&m.mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            OBJMesh {
                name: file_name.to_string(),
                vertex_buffer,
                index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            }
        })
        .collect::<Vec<_>>();

    Ok(OBJModel { meshes, materials })
}
