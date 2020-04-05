// Basic GGez requires EventHandler, Context and Graphics.
use ggez::event;
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
// Some extra imports
use ggez::conf;
use ggez::input::keyboard;
use ggez::nalgebra::Point2;
use ggez::timer;

// Static vars
const TILE_OFFSET_X: f32 = -40.0;
const TILE_OFFSET_Y: f32 = 0.0;
const TILE_COUNT_X: u8 = 17;
const TILE_COUNT_Y: u8 = 9;
const TILE_WIDTH: u8 = 80;
const TILE_HEIGHT: u8 = 80;
const PLAYER_WIDTH: u8 = 40;
const PLAYER_HEIGHT: u8 = 40;
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
            vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
            vec![0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let player = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0f32, 0f32, PLAYER_WIDTH as f32, PLAYER_HEIGHT as f32),
            graphics::Color::new(0.9, 0.56, 0.9, 1.0),
        )?;
        Ok(GameState {
            bg_color: graphics::Color::new(0.878, 0.878, 0.878, 1.0),
            tile_map_start_x: TILE_OFFSET_X, // Will move half tile left, so that more tiles are visible
            tile_map_start_y: TILE_OFFSET_Y,
            tile_map,
            player,
            player_pos: Point2::new(2.0 * TILE_WIDTH as f32, 4.0 * TILE_HEIGHT as f32),
        })
    }
}

fn is_valid_location(state: &GameState, pos_x: &f32, pos_y: &f32) -> bool {
    let player_tile_x_start = (pos_x / TILE_WIDTH as f32).floor();
    let player_tile_x_end = ((pos_x + PLAYER_WIDTH as f32) / TILE_WIDTH as f32).floor();
    let player_tile_y_start = (pos_y / TILE_HEIGHT as f32).floor();
    let player_tile_y_end = ((pos_y + PLAYER_HEIGHT as f32) / TILE_HEIGHT as f32).floor();

    if player_tile_x_start >= 0.0
        && player_tile_x_end < TILE_COUNT_X as f32
        && player_tile_y_start >= 0.0
        && player_tile_y_end < TILE_COUNT_Y as f32
    {
        let vertex_top_left =
            state.tile_map[player_tile_y_start as usize][player_tile_x_start as usize];
        let vertex_top_right =
            state.tile_map[player_tile_y_start as usize][player_tile_x_end as usize];
        let vertex_bottom_right =
            state.tile_map[player_tile_y_end as usize][player_tile_x_start as usize];
        let vertex_bottom_left =
            state.tile_map[player_tile_y_end as usize][player_tile_x_end as usize];

        return vertex_top_left == 1
            && vertex_top_right == 1
            && vertex_bottom_right == 1
            && vertex_bottom_left == 1;
    }

    return false;
}

// We need our state to have implemented the EventHandler Trait
impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, FPS) {
            let old_pos_x = self.player_pos.x;
            let old_pos_y = self.player_pos.y;
            let mut del_x: f32 = 0.0;
            let mut del_y: f32 = 0.0;
            // These can be considered as Level-Triggered Key Presses
            if keyboard::is_key_pressed(ctx, event::KeyCode::Up) {
                del_y = -1.0;
            }
            if keyboard::is_key_pressed(ctx, event::KeyCode::Right) {
                del_x = 1.0;
            }
            if keyboard::is_key_pressed(ctx, event::KeyCode::Down) {
                del_y = 1.0;
            }
            if keyboard::is_key_pressed(ctx, event::KeyCode::Left) {
                del_x = -1.0;
            }

            let new_pos_x = old_pos_x + del_x;
            let new_pos_y = old_pos_y + del_y;

            if is_valid_location(self, &new_pos_x, &new_pos_y) {
                self.player_pos.x = new_pos_x;
                self.player_pos.y = new_pos_y;
            }
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
                if *element == 1 {
                    color = graphics::Color::new(0.5, 0.5, 0.5, 1.0);
                }
                let rect = graphics::Rect::new(
                    col as f32 * TILE_WIDTH as f32,
                    row as f32 * TILE_HEIGHT as f32,
                    TILE_WIDTH as f32,
                    TILE_HEIGHT as f32,
                );
                let r1 =
                    graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
                graphics::draw(
                    ctx,
                    &r1,
                    graphics::DrawParam::default()
                        .dest(Point2::new(self.tile_map_start_x, self.tile_map_start_y)),
                )?;
            }
        }

        graphics::draw(
            ctx,
            &self.player,
            graphics::DrawParam::default().dest(Point2::new(
                self.player_pos.x + TILE_OFFSET_X,
                self.player_pos.y + TILE_OFFSET_Y,
            )),
        )?;

        graphics::present(ctx)?; // Show the Background
        timer::yield_now();
        Ok(())
    }

    // ggez keyboard events are edge-triggered, thus we need to check
    // every key event to verify if multiple keys are pressed and make the changes accordingly.
    // These are Edge-Triggered Key Presses
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
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
        .build()
        .unwrap();

    // Instantiate Game State:
    let mut initial_state = GameState::new(ctx)?;

    // Run the Event Loop
    event::run(ctx, event_loop, &mut initial_state)
}
