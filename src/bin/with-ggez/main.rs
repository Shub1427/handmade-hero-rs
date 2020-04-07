use std::cmp;
// Basic GGez requires EventHandler, Context and Graphics.
use ggez::event;
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
// Some extra imports
use ggez::conf;
use ggez::input::keyboard;
use ggez::nalgebra::Point2;
use ggez::timer;

mod tile_map;
use tile_map::{TileMap, TILE_HEIGHT, TILE_WIDTH};

// Static vars
const TILE_OFFSET_X: f32 = -40.0;
const TILE_OFFSET_Y: f32 = 0.0;
const TILE_COUNT_X: u8 = 17;
const TILE_COUNT_Y: u8 = 9;
const PLAYER_WIDTH: u8 = 40;
const PLAYER_HEIGHT: u8 = 40;
const FPS: u32 = 60;

struct GameState {
    bg_color: graphics::Color,
    tile_maps: Vec<Vec<TileMap>>,
    current_world_tile_x: usize,
    current_world_tile_y: usize,
    player_in_tile_x: usize,
    player_in_tile_y: usize,
    player: graphics::Mesh,
    player_pos: Point2<f32>,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        // #region Tile Map Instantiations
        let tile_map00 = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
            vec![0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let tile_map10 = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0],
            vec![0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1],
            vec![0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let tile_map11 = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0],
            vec![0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        let tile_map01 = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0],
            vec![0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0],
            vec![0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        // #endregion Tile Map Instantiations
        let player = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0f32, 0f32, PLAYER_WIDTH as f32, PLAYER_HEIGHT as f32),
            graphics::Color::new(0.9, 0.56, 0.9, 1.0),
        )?;
        let player_pos = Point2::new(2.0 * TILE_WIDTH as f32, 4.0 * TILE_HEIGHT as f32);
        Ok(GameState {
            bg_color: graphics::Color::new(0.878, 0.878, 0.878, 1.0),
            tile_maps: vec![
                vec![
                    TileMap::new(tile_map00, TILE_OFFSET_X, TILE_OFFSET_Y),
                    TileMap::new(tile_map01, TILE_OFFSET_X, TILE_OFFSET_Y),
                ],
                vec![
                    TileMap::new(tile_map10, TILE_OFFSET_X, TILE_OFFSET_Y),
                    TileMap::new(tile_map11, TILE_OFFSET_X, TILE_OFFSET_Y),
                ],
            ],
            current_world_tile_x: 0,
            current_world_tile_y: 0,
            player_in_tile_x: (player_pos.x / TILE_WIDTH as f32).floor() as usize,
            player_in_tile_y: (player_pos.y / TILE_HEIGHT as f32).floor() as usize,
            player,
            player_pos,
        })
    }
}

// Keeping and manipulating a 2D array as World Tile Map, because, if I keep World map
// as a 1D array of pointers to Tile Maps, then it is difficult to move to a particular
// direction, without actually knowing the direction of the player moving.
fn is_valid_location_in_world(world: &mut GameState, pos_x: &mut f32, pos_y: &mut f32) -> bool {
    let mut player_tile_x_start = (*pos_x / TILE_WIDTH as f32).floor();
    let mut player_tile_x_end = ((*pos_x + PLAYER_WIDTH as f32) / TILE_WIDTH as f32).floor();
    let mut player_tile_y_start = (*pos_y / TILE_HEIGHT as f32).floor();
    let mut player_tile_y_end = ((*pos_y + PLAYER_HEIGHT as f32) / TILE_HEIGHT as f32).floor();

    // Modify the the tile map pointer in world map.
    if player_tile_x_start < 0.0 {
        player_tile_x_start = TILE_COUNT_X as f32 + player_tile_x_start;
        player_tile_x_end = player_tile_x_start;
        world.current_world_tile_x = cmp::max(
            world.current_world_tile_x - 1,
            0
        );
        *pos_x = player_tile_x_start * TILE_WIDTH as f32;
    }
    if player_tile_x_end >= TILE_COUNT_X as f32 {
        player_tile_x_end = player_tile_x_end - TILE_COUNT_X as f32;
        player_tile_x_start = player_tile_x_end;
        // We need to clip the indexes to be within limits or array size.
        // This can be greatly optimized, but will do that later.
        world.current_world_tile_x = cmp::min(
            world.current_world_tile_x + 1,
            world.tile_maps[world.current_world_tile_y].len() - 1
        );
        *pos_x = player_tile_x_end * TILE_WIDTH as f32;
    }
    if player_tile_y_start < 0.0 {
        // Since we are changing the whole screen, y_start and y_end should match after this.
        player_tile_y_start = TILE_COUNT_Y as f32 + player_tile_y_start;
        player_tile_y_end = player_tile_y_start;
        world.current_world_tile_y = cmp::max(
            world.current_world_tile_y - 1,
            0
        );
        *pos_y = player_tile_y_start * TILE_HEIGHT as f32;
    }
    if player_tile_y_end >= TILE_COUNT_Y as f32 {
        player_tile_y_end = player_tile_y_end - TILE_COUNT_Y as f32;
        player_tile_y_start = player_tile_y_end;
        world.current_world_tile_y = cmp::min(
            world.current_world_tile_y + 1,
            world.tile_maps.len() - 1
        );
        *pos_y = player_tile_y_end * TILE_HEIGHT as f32;
    }

    let tile_map = &mut world.tile_maps[world.current_world_tile_y][world.current_world_tile_x];

    if player_tile_x_start >= 0.0
        && player_tile_x_end < TILE_COUNT_X as f32
        && player_tile_y_start >= 0.0
        && player_tile_y_end < TILE_COUNT_Y as f32
    {
        let vertex_top_left =
            tile_map.tile_map[player_tile_y_start as usize][player_tile_x_start as usize];
        let vertex_top_right =
            tile_map.tile_map[player_tile_y_start as usize][player_tile_x_end as usize];
        let vertex_bottom_right =
            tile_map.tile_map[player_tile_y_end as usize][player_tile_x_start as usize];
        let vertex_bottom_left =
            tile_map.tile_map[player_tile_y_end as usize][player_tile_x_end as usize];

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

            let mut new_pos_x = old_pos_x + del_x;
            let mut new_pos_y = old_pos_y + del_y;

            if is_valid_location_in_world(
                self,
                &mut new_pos_x,
                &mut new_pos_y,
            ) {
                self.player_pos.x = new_pos_x;
                self.player_pos.y = new_pos_y;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear the screen for each draw with Background Color
        graphics::clear(ctx, self.bg_color);

        // for tile_map in self.tile_maps.iter_mut() {
        //     tile_map.draw(ctx)?;
        // }
        self.tile_maps[self.current_world_tile_y][self.current_world_tile_x].draw(ctx)?;

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
