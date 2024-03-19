use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use sdl2::{gfx::primitives::DrawRenderer, rect::Rect, render::Canvas, video::Window};

use crate::Color;

use super::Component;

enum TextAlignment {
    Left,
    Center,
    Right,
}

enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}

pub struct Text {
    text: String,
    x: i32,
    y: i32,
    color: Color,
    font_size: u16,
    alignment: TextAlignment,
    vertical_alignment: VerticalAlignment,
    font_style: sdl2::ttf::FontStyle,
    shrink_to_fit: bool,
}

impl Text {
    pub fn new<S: ToString>(text: S) -> Self {
        Text {
            text: text.to_string(),
            x: 0,
            y: 0,
            color: Color::RGB(255, 255, 255),
            font_size: 32,
            alignment: TextAlignment::Left,
            font_style: sdl2::ttf::FontStyle::NORMAL,
            shrink_to_fit: true,
            vertical_alignment: VerticalAlignment::Top,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn font_size(mut self, size: u16) -> Self {
        self.font_size = size;
        self
    }

    pub fn align_left(mut self) -> Self {
        self.alignment = TextAlignment::Left;
        self
    }

    pub fn align_center(mut self) -> Self {
        self.alignment = TextAlignment::Center;
        self
    }

    pub fn align_right(mut self) -> Self {
        self.alignment = TextAlignment::Right;
        self
    }

    pub fn vertical_align_top(mut self) -> Self {
        self.vertical_alignment = VerticalAlignment::Top;
        self
    }

    pub fn vertical_align_middle(mut self) -> Self {
        self.vertical_alignment = VerticalAlignment::Middle;
        self
    }

    pub fn vertical_align_bottom(mut self) -> Self {
        self.vertical_alignment = VerticalAlignment::Bottom;
        self
    }

    pub fn font_style(mut self, style: sdl2::ttf::FontStyle) -> Self {
        self.font_style = style;
        self
    }

    pub fn as_box(self) -> Box<dyn Component> {
        Box::new(self)
    }
}

impl Component for Text {
    fn render(&self, context: &super::RenderingContext) -> Result<(), String> {
        let mut canvas = context.canvas.borrow_mut();

        canvas.set_draw_color(self.color);

        let texture_creator = canvas.texture_creator();

        // need to put this in a font caching thing
        let ttf_context = context.ttf_context.borrow();
        let mut font = ttf_context.load_font("./Karla-VariableFont_wght.ttf", self.font_size)?;

        font.set_style(self.font_style);

        let surface = font
            .render(&self.text)
            .blended(self.color)
            .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let max_width = canvas.viewport().width();
        let max_height = canvas.viewport().height();

        let rect = if self.shrink_to_fit {
            constrain_rect(
                surface.width(),
                surface.height(),
                max_width as u32,
                max_height as u32,
            )
        } else {
            Rect::new(self.x, self.y, surface.width(), surface.height())
        };

        let rect = match self.alignment {
            TextAlignment::Left => rect,
            TextAlignment::Center => {
                let x = self.x + (max_width as i32 - rect.width() as i32) / 2;
                Rect::new(x, self.y, rect.width(), rect.height())
            }
            TextAlignment::Right => {
                let x = self.x + (max_width as i32 - rect.width() as i32);
                Rect::new(x, self.y, rect.width(), rect.height())
            }
        };

        let rect = match self.vertical_alignment {
            VerticalAlignment::Top => rect,
            VerticalAlignment::Middle => {
                let y = self.y + (max_height as i32 - rect.height() as i32) / 2;
                Rect::new(rect.x(), y, rect.width(), rect.height())
            }
            VerticalAlignment::Bottom => {
                let y = self.y + (max_height as i32 - rect.height() as i32);
                Rect::new(rect.x(), y, rect.width(), rect.height())
            }
        };

        canvas.copy(&texture, None, rect)?;

        Ok(())
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
