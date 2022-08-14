use std::time::Duration;

use crate::{
    frame::{Drawable, Frame},
    invaders::Invaders,
    shot::Shot,
    NUM_COLS, NUM_ROWS,
};

const MAX_SHOTS: usize = 2;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < MAX_SHOTS {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.shots.iter_mut().for_each(|shot| {
            shot.update(delta);
        });
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit_something = false;
        let active_shots = self.shots.iter_mut().filter(|shot| !shot.exploding);

        for shot in active_shots {
            hit_something = invaders.kill_invader_at(shot.x, shot.y);
            if hit_something {
                shot.explode();
                break;
            }
        }

        hit_something
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";

        self.shots.iter().for_each(|shot| {
            shot.draw(frame);
        });
    }
}
