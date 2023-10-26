use std::{env, path};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, GameResult};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use crate::test_cache::TestCacheApp;

mod scene;
mod asset;
// mod cache_bak;
mod cache_1;
mod test_cache;
mod layer;
mod cache;
mod easing;

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%F %T%.6f"))
    }
}

fn main() -> GameResult {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "error,file=info,map_dist=debug")
    }
    tracing_subscriber::fmt::fmt()
        .with_timer(LocalTimer)
        .with_thread_names(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("D32", "iX")
        .add_resource_path(resource_dir)
        .window_setup(WindowSetup::default().title("D32"))
        .window_mode(WindowMode::default().dimensions(1920.0, 1280.0));

    let (mut ctx, event_loop) = cb.build()?;

    let app = TestCacheApp::new("/Users/vt/Documents/LegendOfMir/", &mut ctx);

    event::run(ctx, event_loop, app)
}


