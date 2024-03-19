use zeal::{
    component::{Button, Text},
    Engine, Color,
};

// TODO: learn about signals! should be *super* easy :)

fn main() -> Result<(), String> {
    let mut engine = Engine::init()?;

    engine.add_component(
        Button::new()
            .text(Text::new("click me").font_size(18))
            .position(200, 300)
            .color(Color::RGB(35, 35, 35))
            .size(zeal::Size::Large)
            .as_box(),
    );

    engine.add_component(
        Text::new("Test")
            .position(100, 200)
            .align_center()
            .color(Color::RGB(0, 255, 0))
            .font_size(48)
            .as_box(),
    );

    engine.run()
}
