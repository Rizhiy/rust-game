use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Image};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use glam::{IVec2, Vec2};
use std::collections::HashMap;

mod world;
use crate::world::{GameState, Tile};

struct Textures {
    tiles: HashMap<Tile, Image>,
}

struct Game {
    state: GameState,
    textures: Textures,
    camera_pos: Vec2,
    zoom: f32,
}

impl Game {
    fn new(seed: u32, textures: Textures) -> Self {
        Self {
            state: GameState::new(seed),
            textures,
            camera_pos: Vec2::default(),
            zoom: 8.0,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let speed = 8.0;
        let vert = Vec2 {
            x: 0.0,
            y: speed / self.zoom,
        };

        let horz = Vec2 {
            x: speed / self.zoom,
            y: 0.0,
        };
        if ctx.keyboard.is_key_pressed(KeyCode::W) {
            self.camera_pos -= vert;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::A) {
            self.camera_pos -= horz;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::S) {
            self.camera_pos += vert;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::D) {
            self.camera_pos += horz;
        }

        Ok(())
    }
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        if y > 0.0 {
            self.zoom *= 1.01;
        }
        if y < 0.0 {
            self.zoom *= 0.99;
        }
        self.zoom = self.zoom.clamp(1.0, 64.0);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        let (width, height) = ctx.gfx.drawable_size();

        let mut x = 0.0;
        let mut y = 0.0;
        while x < width {
            while y < height {
                let pos = Vec2 {
                    x: x / self.zoom,
                    y: y / self.zoom,
                };
                let draw_pos = pos + self.camera_pos;
                let tile = self.state.tile_at(IVec2 {
                    x: draw_pos.x as i32,
                    y: draw_pos.y as i32,
                });
                let image = &self.textures.tiles[&tile];
                let scale = self.zoom / 8.0;
                canvas.draw(
                    image,
                    DrawParam::default().dest([x, y]).scale([scale, scale]),
                );
                y += self.zoom;
            }
            y = 0.0;
            x += self.zoom;
        }

        canvas.finish(ctx)
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("rust-game", "Artem Vasenin")
        .add_resource_path("./resources")
        .build()?;

    let grass = Image::from_path(&ctx, "/textures/grass.png")?;
    let water = Image::from_path(&ctx, "/textures/water.png")?;
    let mut textures = HashMap::new();
    textures.insert(Tile::Grass, grass);
    textures.insert(Tile::Water, water);
    let game = Game::new(0, Textures { tiles: textures });
    event::run(ctx, event_loop, game)
}
