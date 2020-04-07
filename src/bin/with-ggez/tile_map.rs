use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

pub const TILE_WIDTH: u8 = 80;
pub const TILE_HEIGHT: u8 = 80;

pub struct TileMap {
    start_x: f32,
    start_y: f32,
    pub tile_map: Vec<Vec<u8>>,
}

impl TileMap {
    pub fn new(tile_map: Vec<Vec<u8>>, start_x: f32, start_y: f32) -> Self {
        TileMap {
            start_x,
            start_y,
            tile_map,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
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
                    graphics::DrawParam::default().dest(Point2::new(self.start_x, self.start_y)),
                )?;
            }
        }
        Ok(())
    }
}
