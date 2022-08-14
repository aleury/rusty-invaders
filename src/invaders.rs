use std::{cmp::max, time::Duration};

use rusty_time::timer::Timer;

use crate::{
    frame::{Drawable, Frame},
    NUM_COLS, NUM_ROWS,
};

enum Direction {
    Left,
    Right,
}

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: Direction,
}

impl Invaders {
    pub fn new() -> Self {
        Self {
            army: Self::new_army(),
            move_timer: Timer::from_millis(2000),
            direction: Direction::Right,
        }
    }

    fn new_army() -> Vec<Invader> {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if Self::is_valid_position(x, y) {
                    army.push(Invader { x, y });
                }
            }
        }
        army
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);

        if self.move_timer.ready {
            self.move_timer.reset();
            let downwards = self.update_army_direction();

            if downwards {
                self.move_army_down();
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
            } else {
                self.move_army();
            }

            return true;
        }

        false
    }

    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        let max_y = self.army.iter().map(|invader| invader.y).max().unwrap_or(0);

        max_y >= NUM_ROWS - 1
    }

    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> bool {
        let collision = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y));

        if let Some(idx) = collision {
            self.army.remove(idx);
            true
        } else {
            false
        }
    }

    fn update_army_direction(&mut self) -> bool {
        let mut move_down = false;
        match self.direction {
            Direction::Left => {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    move_down = true;
                    self.direction = Direction::Right;
                }
            }
            Direction::Right => {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    move_down = true;
                    self.direction = Direction::Left;
                }
            }
        };
        move_down
    }

    fn move_army_down(&mut self) {
        for invader in self.army.iter_mut() {
            invader.y += 1;
        }
    }

    fn move_army(&mut self) {
        for invader in self.army.iter_mut() {
            match self.direction {
                Direction::Left => invader.x -= 1,
                Direction::Right => invader.x += 1,
            }
        }
    }

    fn is_valid_position(x: usize, y: usize) -> bool {
        Self::is_in_bounds(x, y) && Self::is_even(x, y)
    }

    fn is_in_bounds(x: usize, y: usize) -> bool {
        let x_in_bounds = (x > 1) && (x < NUM_COLS - 2);
        let y_in_bounds = (y > 0) && (y < 9);
        return x_in_bounds && y_in_bounds;
    }

    fn is_even(x: usize, y: usize) -> bool {
        (x % 2 == 0) && (y % 2 == 0)
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            let time_ratio =
                self.move_timer.time_left.as_secs_f32() / self.move_timer.duration.as_secs_f32();

            frame[invader.x][invader.y] = if time_ratio > 0.5 { "x" } else { "+" };
        }
    }
}
