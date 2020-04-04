// This is replica of the implementation doen by [Gray Olson](https://github.com/termhn)
// Here - https://github.com/ggez/ggez/pull/178/files
use ggez::graphics;
use ggez::{ Context, GameResult };

// pub enum GradientDirection {
//     Horizontal,
//     Vertical
// }

pub trait Gradient {
    fn gradient(
        ctx: &mut Context,
        colors: &[graphics::Color],
    ) -> GameResult<graphics::Image>;
}

impl Gradient for graphics::Image {
    fn gradient(
        ctx: &mut Context,
        colors: &[graphics::Color],
    ) -> GameResult<graphics::Image> {
        let buf_size = colors.len();
        let mut buffer: Vec<u8> = Vec::with_capacity(buf_size);
        for color in colors {
            let color = color.to_rgba();
            buffer.extend([color.0, color.1, color.2, color.3].iter());
        }
        // Horizontal Gradient.
        let mut result = graphics::Image::from_rgba8(ctx, 1, buf_size as u16, &buffer)?;
        result.set_filter(graphics::FilterMode::Linear);
        Ok(result)
    }
}
