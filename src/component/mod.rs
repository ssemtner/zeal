use std::{any::Any, cell::RefCell, rc::Rc};

use sdl2::{rect::Rect, render::Canvas, video::Window};

mod button;
mod text;

pub use button::Button;
pub use text::Text;

use crate::engine::ClickHandler;

use super::engine::RenderingContext;

pub trait Component {
    fn click_handlers(&self) -> Vec<ClickHandler> {
        vec![]
    }
    fn render(&self, canvas: &RenderingContext) -> Result<(), String>;
}
