pub mod background;
pub mod bird;
pub mod ground;
pub mod tube;

use background::Background;
use bird::Bird;
use ground::Ground;
use macroquad::{
    audio::{load_sound, play_sound, PlaySoundParams, Sound},
    prelude::{is_key_pressed, KeyCode, Rect, BLACK},
    text::{draw_text, measure_text},
};
use tube::Tubes;

use crate::helpers::draw_debug_state;

#[derive(Debug)]
pub enum GameState {
    Menu,
    Game,
    Pause,
    Collision,
}

#[derive(Debug)]
pub struct Manager {
    bird: Bird,
    background: Background,
    ground: Ground,
    tubes: Tubes,
    sound: Sound,
    pub collision: bool,
    pub score: u32,
    pub high_score: u32,
}

impl Manager {
    pub async fn new() -> Self {
        let bird = Bird::new().await;
        let background = Background::new().await;
        let ground = Ground::new().await;
        let tubes = Tubes::new().await;
        let s = load_sound("res/music.ogg").await.unwrap();
        play_sound(
            s,
            PlaySoundParams {
                looped: true,
                volume: 0.20f32,
            },
        );

        Self {
            bird,
            background,
            ground,
            tubes,
            sound: s,
            collision: false,
            score: 0,
            high_score: 0,
        }
    }

    pub fn update(&mut self, game_state: GameState) {
        match game_state {
            GameState::Menu => {
                self.tubes.update(None);
                self.ground.update(None);
            }

            GameState::Game => {
                if is_key_pressed(KeyCode::Space) {
                    self.bird.jump()
                }
                self.bird.update();
                self.tubes.update(Some(Rect::new(
                    self.bird.position.x,
                    self.bird.position.y,
                    self.bird.rect.w,
                    self.bird.rect.h,
                )));
                self.score = self.tubes.score;
                self.collision = self.tubes.collision;

                if !self.collision {
                    self.ground.update(Some(Rect::new(
                        self.bird.position.x,
                        self.bird.position.y,
                        self.bird.rect.w,
                        self.bird.rect.h,
                    )));
                    self.collision = self.ground.collision;
                }

                if self.collision && self.score > self.high_score {
                    self.high_score = self.score
                }
            }

            _ => (),
        }
    }

    pub fn draw(&mut self, game_state: GameState) {
        self.background.draw();

        match game_state {
            GameState::Menu => {
                self.tubes.draw();
                self.ground.draw();
            }

            GameState::Game => {
                self.tubes.draw();
                self.ground.draw();
                self.bird.draw();
            }

            GameState::Pause => {
                self.tubes.draw();
                self.ground.draw();
                self.bird.draw();
            }

            GameState::Collision => {
                self.tubes.draw();
                self.ground.draw();
                self.bird.draw();
            }
        }
    }

    pub fn draw_score(&self, text: &str, score: u32, x: f32) {
        let text = format!("{}:{}", text, score);

        let d = measure_text(&text, None, 30u16, 1f32);

        draw_text(&text, x, 30f32, 30f32, BLACK);
    }

    pub fn reset(&mut self) {
        self.bird.reset();
        self.tubes.reset();
        self.ground.reset();
        self.score = 0
    }
}
