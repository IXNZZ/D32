use std::ops::Deref;
use std::time::Instant;
use ggez::event::EventHandler;
use ggez::{Context, GameError};
use ggez::graphics::{Canvas, Color, DrawParam};
use crate::cache;
use crate::layer;
use crate::layer::map::MapLayer;
// use crate::cache_1::ImageCacheManager;

pub struct TestCacheApp {
    cache: cache::ImageCache,
    map_layer: layer::map::MapLayer,
}

impl TestCacheApp {
    pub fn new(base_dir: &str, ctx: &mut Context) -> Self {
        let scale_factor = ctx.gfx.window().scale_factor();
        let size = ctx.gfx.window().inner_size();
        let (draw_width, draw_height) = ctx.gfx.drawable_size();
        let monitor_size = ctx.gfx.window().current_monitor().unwrap().size();
        println!("dw: {}, dh: {}", draw_width, draw_height);
        println!("monitor_size: {:?}", monitor_size);
        println!("size: {:?}, scale_factor: {}", size, scale_factor);
        let new_width = size.width as f64 * scale_factor;
        let new_height = size.height as f64 * scale_factor;
        if new_width < monitor_size.width as f64 && new_height < monitor_size.height as f64 {
            ctx.gfx.set_drawable_size(new_width as f32, new_height as f32).unwrap();
        }
        let mut map = MapLayer::new((base_dir.to_lowercase() + "map").as_str(), 10, 1, "n3", "测试", draw_width, draw_height);
        map.jump_by_tile(333, 333, 0, 0);
        TestCacheApp {
            cache: cache::ImageCache::new(base_dir.to_lowercase() + "data"),
            map_layer: map,
        }
    }
}

impl EventHandler<GameError> for TestCacheApp {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // let now = Instant::now();
        // let key = cache::CacheKey::from(1, 1, 1, 10, 3, 1, 5);
        // self.cache.load_key(key);
        // let key = cache::CacheKey::from(1, 1, 1, 10, 3, 1, 50);
        // self.cache.load_key(key);
        //
        // // let (image_key, meta_key, image_count) = cache::split_key(key);
        // if let Some(img) = self.cache.get(ctx,&key.get_data_key()) {
        //     let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        //     canvas.draw(&img.image(), DrawParam::default());
        //     canvas.finish(ctx).unwrap();
        // }
        // println!("inst: {:?}", now.elapsed());
            let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        //     canvas.draw(&img.image(), DrawParam::default());
        self.map_layer.draw_tile(&mut canvas, ctx, &mut self.cache, 0x1FF);
        canvas.finish(ctx).unwrap();
        Ok(())
        // Err(GameError::ConfigError(String::new()))
    }
}