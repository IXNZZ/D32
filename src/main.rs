use std::{env, path};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{GameResult};
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use crate::state::State;

mod scene;
mod asset;
// mod cache_bak;
mod cache;
mod easing;
mod control;
mod component;
mod state;
mod app;
mod event;
mod net;

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%F %T%.6f"))
    }
}

fn main() -> GameResult {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "error,D32=info,map_dist=debug")
    }
    tracing_subscriber::fmt::fmt()
        .with_timer(LocalTimer)
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let resource_dir = if let Ok(manifest_dir) = env::var("GAME_DIR") {
        path::PathBuf::from(manifest_dir)
    } else {
        path::PathBuf::from("./")
    };
    info!("RUN DIR: {:?}", resource_dir);
    let cb = ggez::ContextBuilder::new("D32", "iX")
        .add_resource_path(resource_dir.clone())
        .window_setup(WindowSetup::default().title("D32"))
        .window_mode(WindowMode::default().dimensions(1920.0, 1280.0));

    let (mut ctx, event_loop) = cb.build()?;

    let mut state = State::new(resource_dir, &mut ctx);

    let mut app = app::App::new(&mut ctx, &mut state);

    // let app = TestCacheApp::new(&resource_dir, &mut ctx);

    event::run(ctx, event_loop, app, state)
}


