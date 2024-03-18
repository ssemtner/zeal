use zeal::{component::{Button, Text}, Engine};

fn main() -> Result<(), String> {
    let mut engine = Engine::init()?;

    engine.add_component(Box::new(Button {
        text: Text {
            text: "Click me".to_string(),
            x: 0,
            y: 0,
            color: zeal::Color::RGB(0, 0, 255),
        },
        x: 100,
        y: 100,
        width: 40,
        height: 40,
        color: zeal::Color::RGB(255, 0, 0),
    }));

    engine.add_component(Box::new(Text {
        text: "Hello, world!".to_string(),
        x: 0,
        y: 0,
        color: zeal::Color::RGB(0, 0, 255),
    }));

    engine.run()
}
