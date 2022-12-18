use macroquad::prelude::*;

use crate::manager::GameState;

pub fn draw_center_text(text: &str) {
    let d = measure_text(text, None, 50u16, 1f32);

    draw_text(
        text,
        screen_width() * 0.5f32 - d.width * 0.5f32,
        screen_height() * 0.5f32 - d.height * 0.5f32,
        50f32,
        BLUE,
    );
}

pub fn draw_debug_state(game_state: &GameState) {
    let text = format!("Game State is {:?}", game_state);

    let d = measure_text(&text, None, 30u16, 1f32);

    draw_text(&text, 5f32, 30f32, 30f32, BLACK);
}

pub fn resolve_collision(tube: &Rect, other: &Rect) -> bool {
    match tube.intersect(*other) {
        Some(intersection) => return true,
        None => return false,
    };
}
