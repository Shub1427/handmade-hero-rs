// Basic GGez requires EventHandler, Context and Graphics.
use ggez::event;
use ggez::graphics;
use ggez::{ Context, ContextBuilder, GameResult };
// Some extra imports
use ggez::conf;
use ggez::timer;
use ggez::input::keyboard;
use ggez::nalgebra::{Point2};

// Static vars
const TILE_WIDTH: u8 = 80;
const TILE_HEIGHT: u8 = 80;
const FPS: u32 = 60;

struct GameState {
    bg_color: graphics::Color,
    tile_map_start_x: f32,
    tile_map_start_y: f32,
    tile_map: Vec<Vec<u8>>,
    player: graphics::Mesh,
    player_pos: Point2<f32>,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let tile_map = vec![
            vec![0,0,0,0, 0,1,0,0, 0,0,0,0, 0,0,0,0, 0],
            vec![0,0,0,0, 1,1,1,0, 0,0,0,1, 1,1,0,0, 0],
            vec![0,0,1,1, 1,0,1,0, 0,0,0,1, 0,1,0,0, 0],
            vec![0,0,1,0, 1,1,1,1, 1,0,0,1, 1,1,1,1, 0],
            vec![0,1,1,1, 1,1,1,1, 1,1,0,1, 1,1,1,1, 1],
            vec![0,0,1,1, 0,1,0,0, 0,1,1,1, 1,1,1,1, 0],
            vec![0,0,0,1, 1,1,1,1, 0,1,1,1, 0,1,0,0, 0],
            vec![0,0,0,0, 0,0,0,1, 1,1,1,1, 1,1,0,0, 0],
            vec![0,0,0,0, 0,0,0,1, 0,0,0,0, 0,0,0,0, 0],
        ];
        let player = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                0f32, 0f32, TILE_WIDTH as f32 / 2.0, TILE_HEIGHT as f32 / 2.0
            ),
            graphics::Color::new(0.9, 0.56, 0.9, 1.0)
        )?;
        Ok(GameState {
            bg_color: graphics::Color::new(0.878, 0.878, 0.878, 1.0),
            tile_map_start_x: -40.0, // Will move half tile left, so that more tiles are visible
            tile_map_start_y: 0.0,
            tile_map,
            player,
            player_pos: Point2::new(2.0 * TILE_WIDTH as f32, 4.0 * TILE_HEIGHT as f32)
        })
    }
}

// We need our state to have implemented the EventHandler Trait
impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Do Nothing for now
        while timer::check_update_time(ctx, FPS) {

        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear the screen for each draw with Background Color
        graphics::clear(ctx, self.bg_color);

        // This is really inefficient, though good for understanding...
        for (row, row_ele) in self.tile_map.iter_mut().enumerate() {
            for (col, element) in row_ele.iter().enumerate() {
                let mut color = graphics::Color::new(1.0, 1.0, 1.0, 1.0);
                if *element == 1 { color = graphics::Color::new(0.5, 0.5, 0.5, 1.0); }
                let rect = graphics::Rect::new(
                    col as f32 * TILE_WIDTH as f32,
                    row as f32 * TILE_HEIGHT as f32,
                    TILE_WIDTH as f32,
                    TILE_HEIGHT as f32
                );
                let r1 =
                    graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
                graphics::draw(ctx, &r1, graphics::DrawParam::default()
                    .dest(Point2::new(self.tile_map_start_x, self.tile_map_start_y))
                )?;
            }
        }

        graphics::draw(ctx, &self.player, graphics::DrawParam::default().dest(self.player_pos))?;

        graphics::present(ctx)?; // Show the Background
        timer::yield_now();
        Ok(())
    }

    // ggez keyboard events are edge-triggered, thus we need to check
    // every key event to verify if multiple keys are pressed and make the changes accordingly.
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
        _repeat: bool
    ) {
        match keycode {
            event::KeyCode::Up => {
                self.player_pos.y -= 8.0;
                if keyboard::is_key_pressed(ctx, event::KeyCode::Left) {
                    self.player_pos.x -= 8.0;
                } else if keyboard::is_key_pressed(ctx, event::KeyCode::Left) {
                    self.player_pos.x += 8.0;
                }
            }
            event::KeyCode::Left => {
                self.player_pos.x -= 8.0;
                if keyboard::is_key_pressed(ctx, event::KeyCode::Up) {
                    self.player_pos.y -= 8.0;
                } else if keyboard::is_key_pressed(ctx, event::KeyCode::Down) {
                    self.player_pos.y += 8.0;
                }
            }
            event::KeyCode::Right => {
                self.player_pos.x += 8.0;
                if keyboard::is_key_pressed(ctx, event::KeyCode::Up) {
                    self.player_pos.y -= 8.0;
                } else if keyboard::is_key_pressed(ctx, event::KeyCode::Down) {
                    self.player_pos.y += 8.0;
                }
            }
            event::KeyCode::Down => {
                self.player_pos.y += 8.0;
                if keyboard::is_key_pressed(ctx, event::KeyCode::Left) {
                    self.player_pos.x -= 8.0;
                } else if keyboard::is_key_pressed(ctx, event::KeyCode::Left) {
                    self.player_pos.x += 8.0;
                }
            }
            event::KeyCode::Escape => event::quit(ctx),
            _ => (), // Do nothing
        }
    }
}

fn main() -> GameResult {
    // We can configure our window dimensions and others, using ggez::config;
    let window_config = conf::Conf::new();
    let win_mode = conf::WindowMode::default();
    let window_config = window_config.window_mode(win_mode.dimensions(1280.0, 720.0));

    // Get GGez Contert and Window Event Loop;
    let (ctx, event_loop) = &mut ContextBuilder::new("Handmade Hero", "Subroto")
        .conf(window_config)
        .build().unwrap();

    // Instantiate Game State:
    let mut initial_state = GameState::new(ctx)?;

    // Run the Event Loop
    event::run(ctx, event_loop, &mut initial_state)
}
