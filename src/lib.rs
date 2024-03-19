pub mod component;
pub mod engine;

pub use engine::Engine;

pub type Color = sdl2::pixels::Color;

pub enum Size {
    Small,
    Medium,
    Large,
    Custom(u32, u32),
}

impl From<(u32, u32)> for Size {
    fn from((width, height): (u32, u32)) -> Self {
        Size::Custom(width, height)
    }
}
