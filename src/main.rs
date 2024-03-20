use zeal::{
    component::{Button, Text},
    Color, Engine,
};

// TODO: learn about signals! should be *super* easy :)

fn main() -> Result<(), String> {
    let mut engine = Engine::init()?;

    let count = engine.runtime.create_state(0);

    engine.add_component(
        Button::new()
            .child(Text::new("Increment").font_size(16))
            .position(200, 300)
            .color(Color::RGB(35, 35, 35))
            .size(zeal::Size::Large)
            .on_click(move || {
                let prev = count.get();
                count.set(prev + 1);
            }),
    );

    engine.add_component(
        Button::new()
            .child("Decrement")
            .position(400, 300)
            .color(Color::RGB(35, 35, 35))
            .size(zeal::Size::Large)
            .on_click(move || {
                let prev = count.get();
                count.set(prev - 1);
            }),
    );

    engine.add_component(move || count.get().to_string());

    // let count = engine.create_signal(0);
    //
    // engine.create_effect(move || {
    //     btn.set_text(&format!("count {}", count.get()));
    // });

    engine.run()
}
