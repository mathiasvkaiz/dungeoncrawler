use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct AnimationConfig {
  pub first_sprite_index: usize,
  last_sprite_index: usize,
  fps: u8,
  frame_timer: Timer,
}

impl AnimationConfig {
  pub fn new(first: usize, last: usize, fps: u8) -> Self {
    Self {
      first_sprite_index: first,
      last_sprite_index: last,
      fps,
      frame_timer: Self::timer_from_fps(fps),
    }
  }

  fn timer_from_fps(fps: u8) -> Timer {
    Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
  }
}