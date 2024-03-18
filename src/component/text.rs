use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::Color;

use super::Component;

pub struct Text {
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Component for Text {
    fn render(&self, context: &super::RenderingContext) {
        let mut canvas = context.canvas.borrow_mut();

        canvas.set_draw_color(self.color);

        // need to put this in a font caching struct
        let ttf_context = context.ttf_context.borrow();
        let font = ttf_context
            .load_font("./Karla-VariableFont_wght.ttf", 32)
            .unwrap();
        let surface = font.render(&self.text).blended(self.color).unwrap();

        let texture_creator = canvas.texture_creator();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let width = canvas.viewport().width();
        let height = canvas.viewport().height();

        canvas
            .copy(
                &texture,
                None,
                constrain_rect(
                    surface.width(),
                    surface.height(),
                    width as u32,
                    height as u32,
                ),
            )
            .unwrap();
    }
}

fn constrain_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            (cons_width, (rect_height as f32 / wr) as u32)
        } else {
            ((rect_width as f32 / hr) as u32, cons_height)
        }
    } else {
        (rect_width, rect_height)
    };

    Rect::new(0, 0, w, h)
}
