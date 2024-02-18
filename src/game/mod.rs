use std::sync::Arc;

use flume::Sender;

use crate::{
    engine::voxel::voxel_state::ChunkUpdates,
    world::{
        position::{RawPosition, RawRotation},
        World,
    },
};

use self::blocks::BlockRegistery;
pub mod blocks;

pub struct GameState {
    pub world: World,
    pub player: Player,
    pub game: Arc<Game>,
}
pub struct Game {
    pub block_registery: BlockRegistery,
    pub game_settings: GameSettings,
    pub chunk_queue: Sender<ChunkUpdates>,
}
pub struct Player {
    pub position: RawPosition,
    pub rotation: RawRotation,
}

pub struct GameSettings {
    pub chunk_render_distance: usize,
}
