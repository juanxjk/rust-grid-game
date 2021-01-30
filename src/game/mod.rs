extern crate piston;
extern crate piston_window;

use piston::EventSettings;
use piston::Events;
use piston_window::clear;
use piston_window::rectangle;
use piston_window::PistonWindow;
use piston_window::WindowSettings;

mod settings;
use settings::GameSettings;

pub struct Game {
    pub settings: GameSettings,
    window: PistonWindow,
}

impl Game {
    pub fn new() -> Self {
        let settings = GameSettings::default();
        let window: PistonWindow = WindowSettings::new("Rust grid game", settings.screen_size)
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
