use zeal::{
    component::{Button, Text},
    Color, Engine,
};

// TODO: learn about signals! should be *super* easy :)

fn main() -> Result<(), String> {
    let mut engine = Engine::init()?;

    let count = engine.runtime.create_state(0);

    engine.add_component(Box::new(
        Button::new()
            .text(Text::new("Increment").font_size(18))
            .position(200, 300)
            .color(Color::RGB(35, 35, 35))
            .size(zeal::Size::Large)
            .on_click(move || {
                let prev = count.get();
                count.set(prev + 1);
            }),
    ));

    engine.add_component(Box::new(
        Button::new()
            .text(Text::new("Decrement").font_size(18))
            .position(400, 300)
            .color(Color::RGB(35, 35, 35))
            .size(zeal::Size::Large)
            .on_click(move || {
                let prev = count.get();
                count.set(prev - 1);
            }),
    ));

    engine.add_component(
        Text::new_reactive(move || format!("count {}", count.get()))
            .position(100, 200)
            .align_center()
            .color(Color::RGB(0, 255, 0))
            .font_size(48)
            .as_box(),
    );

    // let count = engine.create_signal(0);
    //
    // engine.create_effect(move || {
    //     btn.set_text(&format!("count {}", count.get()));
    // });

    engine.run()
}
