/// Canvas package for drawing stuff on canvas
/// This means that we can be artists now!
mod canvas;
mod color;
mod softgl;
mod stroke;
mod svg_output;
mod transform;

pub use canvas::Canvas;
pub use color::Color;
pub use stroke::Stroke;
pub use svg_output::SvgOutput;
