use std::ops::Deref;
use std::path::PathBuf;
use std::time::Instant;
use ggez::event::{EventHandler, MouseButton};
use ggez::{Context, GameError};
use ggez::graphics::{Canvas, Color, DrawParam};
use crate::cache;
use crate::draw;
use crate::draw::map::MapDraw;
// use crate::cache_1::ImageCacheManager;

pub struct TestCacheApp {
    cache: cache::ImageCache,
    map_layer: MapDraw,
    center_x: f32,
    center_y: f32,
}

impl TestCacheApp {
    pub fn new(path: &PathBuf, ctx: &mut Context) -> Self {
        // ctx.fs.resources_dir()
        let scale_factor = ctx.gfx.window().scale_factor();
        let size = ctx.gfx.window().inner_size();
        let (draw_width, draw_height) = ctx.gfx.drawable_size();
        // let monitor_size = ctx.gfx.window().current_monitor().unwrap().size();
        // println!("dw: {}, dh: {}", draw_width, draw_height);
        // println!("monitor_size: {:?}", monitor_size);
        // println!("size: {:?}, scale_factor: {}", size, scale_factor);
        // let new_width = size.width as f64 * scale_factor;
        // let new_height = size.height as f64 * scale_factor;
        // if new_width < monitor_size.width as f64 && new_height < monitor_size.height as f64 {
        //     ctx.gfx.set_drawable_size(new_width as f32, new_height as f32).unwrap();
        // }
        // println!("{:?}", ctx.fs.user_config_dir());
        // println!("{:?}", ctx.fs.user_data_dir());
        // println!("{:?}", ctx.fs.resources_dir());
        let mut map = MapDraw::new(&path, 10, 1, "n3", "测试", draw_width, draw_height);
        map.jump_by_tile(333, 333, 0, 0);
        TestCacheApp {
            cache: cache::ImageCache::new(path.join("data")),
            map_layer: map,
            center_x: draw_width / 2.,
            center_y: draw_height / 2.,
        }
    }
}

impl EventHandler<GameError> for TestCacheApp {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let now = Instant::now();
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
        let mut canvas = Canvas::from_frame(ctx, Color::new(0.1, 0.2, 0.3, 1.0));
        //     canvas.draw(&img.image(), DrawParam::default());
        self.map_layer.draw_tile(&mut canvas, ctx, &mut self.cache, 0x1FF);
        self.map_layer.draw_objects(ctx, &mut canvas, &mut self.cache);

        ctx.gfx.set_window_title(&format!(
            "D32 - {:.0} FPS",
            ctx.time.fps(),
        ));
        // println!("inst: {:?}", now.elapsed());
        canvas.finish(ctx).unwrap();
        Ok(())
        // Err(GameError::ConfigError(String::new()))
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> Result<(), GameError> {
        let angle = angle2(self.center_x, self.center_y, _x, _y);
        let sharing = sharing2(angle, 8.0);
        // let angle2 = angle + if angle < 0. {  360.0 } else { 0. };
        // println!("first: {}", angle2);
        // let angle2 = angle2 + 90.0 + 360.0 / 2.0 / 8.0;
        // println!("sharing: {}, angle: {}, an2: {}", sharing, angle, angle2);
        
        Ok(())
    }
}

fn angle2(src_x: f32, src_y: f32, dst_x: f32, dst_y: f32) -> f32 {
    (dst_y - src_y).atan2(dst_x - src_x) * 57.295776
}

fn sharing2(angle: f32, sharing: f32) -> f32 {
    // let s = 90.0 + 360.0 / 2.0 / sharing as f32;
    let angle = angle + 90.0 + 360.0 / 2.0 / sharing;
    let angle = angle + if angle < 0. { 360.0 } else { 0. };
    ((angle) / (360. / sharing) + 1.0).floor()
}