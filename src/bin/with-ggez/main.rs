// Basic GGez requires EventHandler, Context and Graphics.
use ggez::event;
use ggez::graphics;
use ggez::{ Context, ContextBuilder, GameResult };
// Some extra imports
use ggez::conf;
use ggez::nalgebra::{Point2, Vector2};

mod gradient;
use gradient::Gradient;

struct GameState {
    bg_color: graphics::Color,
    gradient: graphics::Image
}

impl GameState {
    fn new(ctx: &mut Context) -> Self {
        let result = graphics::Image::gradient(ctx, &[
            graphics::Color::from_rgba(183, 28, 28, 255),
            graphics::Color::from_rgba(194, 24, 91, 255),
            graphics::Color::from_rgba(156, 39, 176, 255),
        ]);
        GameState {
            bg_color: graphics::Color::new(0.878, 0.878, 0.878, 1.0),
            gradient: result.unwrap() // This is a 1X3 size image.
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

        // To make my gradient scale the window, I need to scale it to
        // match window, width/height.
        graphics::draw(
            ctx,
            &self.gradient,
            graphics::DrawParam::new()
                .dest(Point2::new(0.0, 0.0))
                .rotation(0f32)
                .scale(Vector2::new(1280.0, 240.0)),
        )?;
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
    let mut initial_state = GameState::new(ctx);

    // Run the Event Loop
    event::run(ctx, event_loop, &mut initial_state)
}
