#![allow(unused)]

mod conf;
mod helpers;
mod manager;

use conf::*;
use helpers::*;
use macroquad::prelude::*;

use manager::GameState;
use manager::Manager;

#[macroquad::main(flappy_bird)]
async fn main() {
    let mut manager = Manager::new().await;
    let mut game_state = GameState::Menu;

    loop {
        match game_state {
            GameState::Menu => {
                manager.update(GameState::Menu);

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                    manager.reset();
                }
            }

            GameState::Game => {
                manager.update(GameState::Game);

                if manager.collision {
                    game_state = GameState::Collision
                }

                if is_key_pressed(KeyCode::P) {
                    game_state = GameState::Pause
                } else if is_key_pressed(KeyCode::M) {
                    game_state = GameState::Menu;
                    manager.reset();
                }
            }

            GameState::Pause => {
                manager.update(GameState::Pause);

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game
                } else if is_key_pressed(KeyCode::M) {
                    game_state = GameState::Menu;
                    manager.reset();
                }
            }

            GameState::Collision => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                    manager.reset();
                } else if is_key_pressed(KeyCode::M) {
                    game_state = GameState::Menu;
                    manager.reset();
                }
            }
        }

        clear_background(WHITE);

        match game_state {
            GameState::Menu => {
                manager.draw(GameState::Menu);
                manager.draw_score("Score", manager.score, 5f32);
                manager.draw_score("High Score", manager.high_score, 180f32);
                draw_center_text("Press SPACE to start")
            }

            GameState::Game => {
                manager.draw(GameState::Game);
                manager.draw_score("Score", manager.score, 5f32);
                manager.draw_score("High Score", manager.high_score, 180f32);
            }

            GameState::Pause => {
                manager.draw(GameState::Pause);
                manager.draw_score("Score", manager.score, 5f32);
                manager.draw_score("High Score", manager.high_score, 180f32);
                draw_center_text("Game is Paused");
            }

            GameState::Collision => {
                manager.draw(GameState::Collision);
                manager.draw_score("Score", manager.score, 5f32);
                manager.draw_score("High Score", manager.high_score, 180f32);
                draw_center_text("You lost");
            }
        }

        next_frame().await;
    }
}
