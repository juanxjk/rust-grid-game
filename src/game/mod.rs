extern crate piston;
extern crate piston_window;
extern crate sdl2_window;

use piston::EventSettings;
use piston::Events;
use piston_window::clear;
use piston_window::rectangle;
use piston_window::Button;
use piston_window::PistonWindow;
use piston_window::WindowSettings;
use sdl2_window::Sdl2Window;

mod settings;
use settings::GameSettings;

pub struct Game {
    pub settings: GameSettings,
    window: PistonWindow<Sdl2Window>,
}

impl Game {
    pub fn new() -> Self {
        let settings = GameSettings::default();
        let window: PistonWindow<Sdl2Window> =
            WindowSettings::new("Rust grid game", settings.screen_size)
                .exit_on_esc(true)
                .build()
                .unwrap();
        Game { settings, window }
    }
    pub fn start(&mut self) {
        self.render();
    }
    pub fn render(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(event) = events.next(&mut self.window) {
            match piston::PressEvent::press_args(&event) {
                Some(Button::Keyboard(piston::Key::Up)) => println!("UP"),
                Some(Button::Keyboard(piston::Key::Right)) => println!("RIGHT"),
                Some(Button::Keyboard(piston::Key::Down)) => println!("DOWN"),
                Some(Button::Keyboard(piston::Key::Left)) => println!("LEFT"),
                Some(Button::Mouse(piston::MouseButton::Left)) => {
                    println!("Pressed Left Mouse button")
                }
                Some(Button::Mouse(piston::MouseButton::Right)) => {
                    println!("Pressed Right Mouse button");
                }
                Some(Button::Keyboard(key)) => println!("Pressed Key: {:#?}.", key),
                _ => {}
            }

            event.mouse_cursor(|x| {
                println!("MouseX: {}", x[0]);
                println!("MouseY: {}", x[1]);
            });

            self.window.draw_2d(&event, |c, g, _| {
                clear([0.5, 0.5, 0.5, 1.0], g);
                rectangle(
                    [1.0, 0.0, 0.0, 1.0],     // red
                    [0.0, 0.0, 100.0, 100.0], // rectangle
                    c.transform,
                    g,
                );
            });
            self.update();
        }
    }

    pub fn update(&mut self) -> () {}
}
