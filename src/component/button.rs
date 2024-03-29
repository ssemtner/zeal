use std::rc::Rc;

use sdl2::{gfx::primitives::DrawRenderer, rect::Rect};

use super::{Component, IntoComponent, Text};
use crate::{engine::ClickHandler, Color, Size};

pub struct Button {
    child: Option<Box<dyn Component>>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: Color,
    click_handlers: Vec<ClickHandler>,
}

impl Button {
    pub fn new() -> Self {
        let (width, height) = size_to_width_height(Size::Medium);

        Button {
            child: None,
            x: 0,
            y: 0,
            width,
            height,
            color: Color::RGB(255, 255, 255),
            click_handlers: vec![],
        }
    }

    pub fn child(mut self, child: impl IntoComponent) -> Self {
        self.child = Some(child.into_component());
        self
    }

    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        (self.width, self.height) = size_to_width_height(size);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn as_box(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn on_click<F: Fn() + 'static>(mut self, handler: F) -> Self {
        let click_handler = ClickHandler {
            region: Rect::new(self.x, self.y, self.width, self.height),
            handler: Rc::new(handler),
        };

        self.click_handlers.push(click_handler);

        self
    }
}

fn size_to_width_height(size: Size) -> (u32, u32) {
    match size {
        Size::Small => (40, 40),
        Size::Medium => (80, 40),
        Size::Large => (120, 40),
        Size::Custom(w, h) => (w, h),
    }
}

impl Component for Button {
    fn click_handlers(&self) -> Vec<ClickHandler> {
        self.click_handlers.clone()
    }

    fn render(&self, context: &super::RenderingContext) -> Result<(), String> {
        context.canvas.borrow_mut().set_draw_color(self.color);
        context.canvas.borrow_mut().fill_rect(Rect::new(
            self.x,
            self.y,
            self.width,
            self.height,
        ))?;

        if let Some(child) = &self.child {
            context.canvas.borrow_mut().set_viewport(Rect::new(
                self.x,
                self.y,
                self.width,
                self.height,
            ));

            // TODO: refactor text so we can get the size of the text surface before copying and
            // rendering to the button. Or pre-add padding to the centering logic and building the
            // button after.
            child.render(context)?;
            context.canvas.borrow_mut().set_viewport(None);
        }

        Ok(())
    }
}

impl IntoComponent for Button {
    fn into_component(self) -> Box<dyn Component> {
        Box::new(self)
    }
}
