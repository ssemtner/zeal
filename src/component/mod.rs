use std::{any::Any, cell::RefCell, rc::Rc};

use sdl2::{rect::Rect, render::Canvas, video::Window};

mod button;
mod text;

pub use button::Button;
pub use text::Text;

use crate::engine::ClickHandler;

use super::engine::RenderingContext;

pub trait Component: IntoComponent {
    fn click_handlers(&self) -> Vec<ClickHandler> {
        vec![]
    }
    fn render(&self, canvas: &RenderingContext) -> Result<(), String>;
}

pub trait IntoComponent {
    fn into_component(self) -> Box<dyn Component>;
}

// IntoComponent for closure returning any type that implements IntoComponent
impl<F, T> IntoComponent for F
where
    F: FnOnce() -> T + 'static,
    T: IntoComponent,
{
    fn into_component(self) -> Box<dyn Component> {
        self().into_component()
    }
}

// macro to define IntoComponent for list of primatives
macro_rules! impl_into_component {
    ($($t:ty),*) => {
        $(
            impl IntoComponent for $t {
                fn into_component(self) -> Box<dyn Component> {
                    Box::new(text::Text::new(self.to_string()))
                }
            }
        )*
    };
}

impl_into_component!(String, &str, i32, u32, i64, u64, f32, f64, bool);
