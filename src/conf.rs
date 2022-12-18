use macroquad::prelude::*;

const WINDOW_TITLE: &str = "Flappy Bird";
pub const WINDOW_WIDTH: i32 = 480i32;
pub const WINDOW_HEIGHT: i32 = 800i32;

pub fn flappy_bird() -> Conf {
    Conf {
        window_title: String::from(WINDOW_TITLE),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}
