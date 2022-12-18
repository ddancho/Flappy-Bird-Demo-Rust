use macroquad::{prelude::*, rand::gen_range};

use crate::{
    conf::{WINDOW_HEIGHT, WINDOW_WIDTH},
    helpers::resolve_collision,
};

const TUBE_SPEED: f32 = 80f32;
const TUBE_GAP: f32 = 4f32;
const NUM_OF_TUBES: usize = 10;

const BIRD_GAP: f32 = 190f32;

#[derive(Clone, Copy, Debug)]
struct Tube {
    rect: Rect,
    check: bool,
}

#[derive(Debug)]
pub struct Tubes {
    texture_2d: Texture2D,
    top: [Tube; 10],
    bottom: [Tube; 10],
    pub collision: bool,
    pub score: u32,
}

impl Tubes {
    pub async fn new() -> Self {
        let t = load_texture("res/tube.png").await.unwrap();
        let (top, bottom) = Self::init(t);

        Self {
            texture_2d: t,
            top,
            bottom,
            collision: false,
            score: 0,
        }
    }

    pub fn update(&mut self, other: Option<Rect>) {
        for i in 0..NUM_OF_TUBES {
            if !self.collision {
                self.top[i].rect.x -= TUBE_SPEED * get_frame_time();
                self.bottom[i].rect.x -= TUBE_SPEED * get_frame_time();

                if (self.top[i].rect.x + self.texture_2d.width() <= 0f32) {
                    let random = gen_range(0f32, 150f32);
                    let gap = TUBE_GAP * 6.5f32;

                    self.top[i].rect.y = 0f32 - random;
                    self.top[i].rect.x = WINDOW_WIDTH as f32 + gap;
                    self.top[i].check = false;

                    self.bottom[i].rect.y =
                        self.top[i].rect.y + self.texture_2d.height() + BIRD_GAP;
                    self.bottom[i].rect.x = WINDOW_WIDTH as f32 + gap;
                    self.bottom[i].check = false;
                }

                if self.top[i].rect.x > 0f32 && self.top[i].rect.x < WINDOW_WIDTH as f32 {
                    let rect = match other {
                        Some(rect) => rect,
                        None => Rect::new(0f32, 0f32, 0f32, 0f32),
                    };

                    if rect.w > 0f32 {
                        if resolve_collision(&self.top[i].rect, &rect)
                            || resolve_collision(&self.bottom[i].rect, &rect)
                        {
                            self.collision = true;
                        }
                    }

                    if !self.collision {
                        if rect.x + rect.w > self.top[i].rect.x + self.top[i].rect.w
                            && !self.top[i].check
                        {
                            self.score += 1;
                            self.top[i].check = true;
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        for i in 0..NUM_OF_TUBES {
            draw_texture_ex(
                self.texture_2d,
                self.top[i].rect.x,
                self.top[i].rect.y,
                WHITE,
                DrawTextureParams {
                    flip_y: true,
                    ..Default::default()
                },
            );
            draw_texture_ex(
                self.texture_2d,
                self.bottom[i].rect.x,
                self.bottom[i].rect.y,
                WHITE,
                DrawTextureParams {
                    ..Default::default()
                },
            );
        }
    }

    pub fn reset(&mut self) {
        (self.top, self.bottom) = Self::init(self.texture_2d);
        self.collision = false;
        self.score = 0;
    }

    fn init(t: Texture2D) -> ([Tube; 10], [Tube; 10]) {
        let mut top: [Tube; 10] = [Tube {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                w: t.width(),
                h: t.height(),
            },
            check: false,
        }; 10];

        let mut bottom: [Tube; 10] = [Tube {
            rect: Rect {
                x: 0f32,
                y: 0f32,
                w: t.width(),
                h: t.height(),
            },
            check: false,
        }; 10];

        for i in 0..NUM_OF_TUBES {
            let random = gen_range(0f32, 150f32);
            let start = WINDOW_WIDTH as f32 * 0.8f32;

            top[i].rect.y = 0f32 - random;
            top[i].rect.x = start + (TUBE_GAP + t.width()) * i as f32;
            top[i].rect.h = t.height();
            top[i].rect.w = t.width();

            bottom[i].rect.y = top[i].rect.y + t.height() + BIRD_GAP;
            bottom[i].rect.x = start + (TUBE_GAP + t.width()) * i as f32;
            bottom[i].rect.h = t.height();
            bottom[i].rect.w = t.width();
        }

        (top, bottom)
    }
}
