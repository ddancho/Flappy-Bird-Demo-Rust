use macroquad::prelude::*;

use crate::conf::{WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Debug)]
pub struct Background {
    texture_2d: Texture2D,
    dest_size: Vec2,
}

impl Background {
    pub async fn new() -> Self {
        let t = load_texture("res/background.png").await.unwrap();

        Self {
            texture_2d: t,
            dest_size: vec2(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            self.texture_2d,
            0f32,
            0f32,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.dest_size),
                ..Default::default()
            },
        )
    }
}
