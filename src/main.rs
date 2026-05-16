use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, GraphicsContext, Image, ImageFormat};
use ggez::{Context, ContextBuilder, GameResult};
use std::time::Instant;

struct MyImage {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl MyImage {
    fn new(width: u32, height: u32) -> Self {
        MyImage {
            width,
            height,
            pixels: vec![0u8; (width * height * 4) as usize],
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let i = ((y * self.width + x) * 4) as usize;
        self.pixels[i] = (color.r * 255.0) as u8;
        self.pixels[i + 1] = (color.g * 255.0) as u8;
        self.pixels[i + 2] = (color.b * 255.0) as u8;
        self.pixels[i + 3] = (color.a * 255.0) as u8;
    }
    fn to_ggez_image(&self, gfx: &GraphicsContext) -> Image {
        Image::from_pixels(
            gfx,
            &self.pixels,
            ImageFormat::Rgba8Unorm,
            self.width,
            self.height,
        )
    }
}

struct Game {
    start_time: Instant,
    size: f32,
    offset: f32,
    speed: f32,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            size: 16.0,
            offset: 0.0,
            speed: 1.0,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.offset = (Instant::now() - self.start_time).as_secs_f32() * self.speed;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let (width, height) = ctx.gfx.drawable_size();

        let mut image = MyImage::new(width as u32, height as u32);

        for x in 0..width as u32 {
            for y in 0..height as u32 {
                let mut color = Color::BLUE;
                if (((x + y) as f32 + self.offset) / self.size) as i32 % 2 == 1 {
                    color = Color::GREEN;
                }
                image.set_pixel(x, y, color);
            }
        }
        canvas.draw(&image.to_ggez_image(&ctx.gfx), DrawParam::default());

        canvas.finish(ctx)
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("rust-game", "Artem Vasenin").build()?;
    let game = Game {
        speed: 50.0,
        ..Default::default()
    };
    event::run(ctx, event_loop, game)
}
