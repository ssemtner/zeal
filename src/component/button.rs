use std::{
    any::Any,
    borrow::BorrowMut,
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use sdl2::{pixels::PixelFormatEnum, rect::Rect, render, surface::Surface, video::Window};

use super::{Component, Text};
use crate::{engine::ClickHandler, Color};

pub struct Button {
    pub text: Text,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: Color,
}

struct ButtonClickEvent {}

impl Component for Button {
    fn click_handlers(&self) -> Vec<ClickHandler> {
        vec![ClickHandler {
            region: Rect::new(self.x, self.y, self.width, self.height),
            handler: Box::new(|| {
                println!("Button clicked!");
            }),
        }]
    }

    fn render(&self, context: &super::RenderingContext) {
        {
            let mut canvas = context.canvas.borrow_mut();

            canvas.set_draw_color(self.color);
            canvas
                .fill_rect(Rect::new(self.x, self.y, self.width, self.height))
                .unwrap();

            canvas.set_viewport(Rect::new(self.x, self.y, self.width, self.height));
        }

        self.text.render(context);

        context.canvas.borrow_mut().set_viewport(None);
    }
}
