use macroquad::{
    audio::{load_sound, play_sound_once, set_sound_volume, Sound},
    prelude::*,
};

const GRAVITY: f32 = 10f32;

#[derive(Debug)]
pub struct Bird {
    texture_2d: Texture2D,
    sound: Sound,
    pub rect: Rect,
    animation_frame: f32,
    animation_speed: f32,
    pub position: Vec2,
    speed: Vec2,
}

impl Bird {
    pub async fn new() -> Self {
        let t = load_texture("res/birdatlas.png").await.unwrap();
        let s = load_sound("res/flap.ogg").await.unwrap();

        Self {
            texture_2d: t,
            sound: s,
            rect: Rect::new(0f32, 0f32, t.width() / 3f32, t.height()),
            animation_frame: 0f32,
            animation_speed: 7f32,
            position: vec2(150f32, 200f32),
            speed: vec2(0f32, 0f32),
        }
    }

    pub fn update(&mut self) {
        self.animation_frame += self.animation_speed * get_frame_time();
        if self.animation_frame > 3f32 {
            self.animation_frame = 0f32
        }

        self.speed.y += GRAVITY * get_frame_time();
        self.position.y += self.speed.y
    }

    pub fn draw(&mut self) {
        self.rect.x = self.animation_frame.trunc() * self.texture_2d.width() / 3f32;

        draw_texture_ex(
            self.texture_2d,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                source: Some(self.rect),
                ..Default::default()
            },
        )
    }

    pub fn jump(&mut self) {
        self.speed.y = -3f32;
        play_sound_once(self.sound);
    }

    pub fn reset(&mut self) {
        self.position = vec2(150f32, 200f32);
        self.speed = vec2(0f32, 0f32)
    }
}
