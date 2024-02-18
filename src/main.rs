#![feature(hash_extract_if)]
use std::{sync::Arc, time::Instant};

use engine::{
    voxel::{texture_atlas::TextureAtlasBuilder, voxel_state::VoxelRendererConfig},
    State,
};
use game::GameState;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};
pub mod engine;
pub mod game;
pub mod world;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    #[cfg(feature = "debug")]
    let mut renderdoc =
        renderdoc::RenderDoc::<renderdoc::V140>::new().expect("Failed to initialize RenderDoc");
    run().await?;
    Ok(())
}
pub fn load_assets_start_internal_server() -> anyhow::Result<(VoxelRendererConfig, GameState)> {
    let (sender, receiver) = flume::bounded(100);
    let texture_atlas = TextureAtlasBuilder::load_from_minecraft_assets()?;
    let block_registery = game::blocks::create_block_registery(texture_atlas.info.clone());
    let game = game::Game {
        block_registery,
        game_settings: game::GameSettings {
            chunk_render_distance: 4,
        },
        chunk_queue: sender,
    };

    let world = world::World::new();
    let player = game::Player {
        position: world::position::RawPosition::new(0.0, 0.0, 0.0),
        rotation: world::position::RawRotation::new(0.0, 0.0),
    };
    let game_state = game::GameState {
        world,
        player,
        game: Arc::new(game),
    };

    Ok((
        VoxelRendererConfig {
            texture_atlas,
            receiver,
        },
        game_state,
    ))
}
pub async fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Kakara.rs")
        .with_inner_size(winit::dpi::LogicalSize::new(854.0, 480.0))
        .build(&event_loop)
        .unwrap();
    window.set_cursor_grab(winit::window::CursorGrabMode::Confined)?;
    window.set_cursor_visible(false);
    window.set_cursor_position(winit::dpi::PhysicalPosition::new(0, 0))?;

    let (config, mut game_state) = load_assets_start_internal_server()?;
    tokio::spawn(async move {
        game_state.world.update_meshes(
            &game_state.player,
            game_state.game.clone(),
            game_state.game.chunk_queue.clone(),
        );
    });
    let mut state = State::new(window, config).await?;
    let mut last_render_time = Instant::now();
    event_loop.run(move |event, window_loop| {
        match event {
            Event::AboutToWait => {
                if !window_loop.exiting() {
                    state.window().request_redraw();
                }
            }
        Event::WindowEvent {
            ref event,
            window_id,
        } =>{
            if window_id != state.window().id()  {
                return
            }
            if state.input(event) {
                return
            }
    match event {
        WindowEvent::CloseRequested => {
            window_loop.exit();
        }
        WindowEvent::KeyboardInput { event, .. } => {
            if event.physical_key.eq(&PhysicalKey::Code(KeyCode::Escape)) {
                window_loop.exit();
            }
        }
        WindowEvent::Resized(physical_size) => {
            state.resize(*physical_size);
        }
        WindowEvent::RedrawRequested => {
            let now = Instant::now();
            let dt = now - last_render_time;
            last_render_time = now;
            state.update(dt);
            //println!("FPS: {}", 1.0 / dt.as_secs_f64());
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if it's lost or outdated
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    state.resize(state.size)
                }
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => {
                    eprintln!("Out of memory");
                    window_loop.exit();
                }
                // We're ignoring timeouts
                Err(wgpu::SurfaceError::Timeout) => {}
            }
        }
        _ => {}
    }
        }
        //Event::MainEventsCleared => state.window().request_redraw(),
        Event::DeviceEvent {
                event: DeviceEvent::MouseMotion{ delta, },
                .. // We're not using device_id currently
            } =>  {
                state.camera_controller.process_mouse(delta.0, delta.1)
            },
        _ => {}
        }
    })?;
    Ok(())
}
