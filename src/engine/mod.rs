use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{pixels::Color, rect::Rect};

use crate::component::Component;

pub struct Engine {
    video_subsystem: sdl2::VideoSubsystem,
    event_subsystem: sdl2::EventSubsystem,
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
    last_mouse_down_pos: Option<(i32, i32)>,
    ttf_context: sdl2::ttf::Sdl2TtfContext,
    components: Vec<Box<dyn Component>>,
    click_handlers: Vec<ClickHandler>,
}

pub struct RenderingContext<'a> {
    pub canvas: RefCell<&'a mut Canvas<Window>>,
    pub ttf_context: RefCell<&'a mut sdl2::ttf::Sdl2TtfContext>,
    pub output_width: u32,
    pub output_height: u32,
    pub mouse_pos: (i32, i32),
}

// struct CustomEvent<'a> {
//     handle: &'a dyn Fn(),
// }

pub struct ClickHandler {
    pub region: Rect,
    pub handler: Box<dyn Fn()>,
}

impl Engine {
    pub fn init() -> Result<Engine, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Test", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.clear();
        canvas.present();

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let event_subsystem = sdl_context.event()?;

        let event_pump = sdl_context.event_pump()?;

        Ok(Engine {
            canvas,
            video_subsystem,
            event_subsystem,
            event_pump,
            last_mouse_down_pos: None,
            ttf_context,
            components: Vec::new(),
            click_handlers: Vec::new(),
        })
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        for click_handler in component.click_handlers() {
            self.click_handlers.push(click_handler);
        }
        self.components.push(component);
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return Ok(()),
                    Event::MouseButtonDown { x, y, .. } => {
                        self.last_mouse_down_pos = Some((x, y));
                    }
                    Event::MouseButtonUp { x, y, .. } => {
                        if let Some((x0, y0)) = self.last_mouse_down_pos {
                            if (x - x0).abs() < 3 && (y - y0).abs() < 3 {
                                for click_handler in &self.click_handlers {
                                    if click_handler.region.contains_point((x, y)) {
                                        (click_handler.handler)();
                                    }
                                }
                            }
                        }
                        self.last_mouse_down_pos = None;
                    }
                    _ => {}
                }
            }
            self.canvas.set_draw_color(Color::RGB(140, 64, 150));
            self.canvas.clear();

            let rendering_context = RenderingContext {
                canvas: RefCell::new(&mut self.canvas),
                ttf_context: RefCell::new(&mut self.ttf_context),
                output_width: 800,
                output_height: 600,
                mouse_pos: {
                    let mouse_state = self.event_pump.mouse_state();
                    (mouse_state.x(), mouse_state.y())
                },
            };

            for component in &self.components {
                component.render(&rendering_context)?;
            }

            self.canvas.present();
        }
    }
}
