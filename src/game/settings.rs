#[derive(Debug)]
pub struct GameSettings {
    pub is_fullscreen: bool,
    pub screen_size: (u32, u32),
}

impl GameSettings {
    pub fn new(is_fullscreen: Option<bool>, screen_size: Option<(u32, u32)>) -> Self {
        GameSettings {
            is_fullscreen: is_fullscreen.unwrap_or(Self::default().is_fullscreen),
            screen_size: screen_size.unwrap_or(Self::default().screen_size),
        }
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            is_fullscreen: false,
            screen_size: (640, 480),
        }
    }
}
