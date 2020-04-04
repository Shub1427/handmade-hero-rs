// Basic GGez requires EventHandler, Context and Graphics.
use ggez::event;
use ggez::graphics;
use ggez::{ Context, ContextBuilder, GameResult };
// Some extra imports
use ggez::conf;

// Static vars
const TILE_WIDTH: u8 = 80;
const TILE_HEIGHT: u8 = 80;

struct GameState {
    bg_color: graphics::Color,
    tile_map: Vec<Vec<u8>>,
}

impl GameState {
    fn new() -> Self {
        let tile_map = vec![
            vec![0,0,0,0, 0,1,0,0, 0,0,0,0, 0,0,0,0],
            vec![0,0,0,0, 1,1,1,0, 0,0,0,1, 1,1,0,0],
            vec![0,0,1,1, 1,0,1,0, 0,0,0,1, 0,1,0,0],
            vec![0,0,1,0, 0,0,0,0, 0,0,0,1, 0,1,0,0],
            vec![0,0,1,1, 0,1,1,1, 1,1,0,1, 0,1,0,0],
            vec![0,0,0,1, 0,1,0,0, 0,1,1,1, 0,1,0,0],
            vec![0,0,0,1, 1,1,0,0, 0,0,0,0, 0,1,0,0],
            vec![0,0,0,0, 0,0,0,1, 1,1,1,1, 1,1,0,0],
            vec![0,0,0,0, 0,0,0,1, 0,0,0,0, 0,0,0,0],
        ];
        GameState {
            bg_color: graphics::Color::new(0.878, 0.878, 0.878, 1.0),
            tile_map
        }
    }
}

// We need our state to have implemented the EventHandler Trait
impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Do Nothing for now
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
                graphics::draw(ctx, &r1, graphics::DrawParam::default())?;
            }
        }

        graphics::present(ctx)?; // Show the Background
        Ok(())
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
    let mut initial_state = GameState::new();

    // Run the Event Loop
    event::run(ctx, event_loop, &mut initial_state)
}
