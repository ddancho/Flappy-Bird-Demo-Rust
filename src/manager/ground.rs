use macroquad::prelude::*;

use crate::{
    conf::{WINDOW_HEIGHT, WINDOW_WIDTH},
    helpers::resolve_collision,
};

const NUM_OF_GROUND: usize = 3;
const GROUND_SPEED: f32 = 100f32;

#[derive(Debug)]
pub struct Ground {
    texture_2d: Texture2D,
    rects: Vec<Rect>,
    pub collision: bool,
}

impl Ground {
    pub async fn new() -> Self {
        let t = load_texture("res/ground.png").await.unwrap();
        let vec: Vec<Rect> = Self::init(t);

        Self {
            texture_2d: t,
            rects: vec,
            collision: false,
        }
    }

    pub fn update(&mut self, other: Option<Rect>) {
        for i in 0..NUM_OF_GROUND {
            if !self.collision {
                self.rects[i].x -= GROUND_SPEED * get_frame_time();

                if self.rects[i].x + self.rects[i].w < 0f32 {
                    self.rects[i].x = self.rects[i].w * 2f32 - 8f32
                }

                let rect = match other {
                    Some(rect) => rect,
                    None => Rect::new(0f32, 0f32, 0f32, 0f32),
                };

                if rect.w > 0f32 {
                    if resolve_collision(&self.rects[i], &rect) {
                        self.collision = true;
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        for i in 0..NUM_OF_GROUND {
            draw_texture(
                self.texture_2d,
                self.rects[i].x,
                WINDOW_HEIGHT as f32 - self.rects[i].h,
                WHITE,
            )
        }
    }

    pub fn reset(&mut self) {
        self.rects = Self::init(self.texture_2d);
        self.collision = false
    }

    fn init(t: Texture2D) -> Vec<Rect> {
        let mut vec: Vec<Rect> = Vec::new();

        for i in 0..NUM_OF_GROUND {
            vec.push(Rect {
                x: t.width() * i as f32,
                y: WINDOW_HEIGHT as f32 - t.height(),
                w: t.width(),
                h: t.height(),
            })
        }

        vec
    }
}
