// Basic GGez requires EventHandler, Context and Graphics.
use ggez::event;
use ggez::graphics;
use ggez::{ Context, ContextBuilder, GameResult };
// Some extra imports
use ggez::conf;

struct GameState {
    bg_color: graphics::Color,
}

impl GameState {
    fn new() -> Self {
        GameState {
            bg_color: graphics::Color::new(0.878, 0.878, 0.878, 1.0)
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
