use std::time::{Duration, Instant};

use crate::controller::{
  joystick::{direction::Direction, vector::Vector},
  settings::{LEFT_STICK_SENSITIVITY, MAX_STICK_TILT, MOUSE_IDLE_TIMEOUT, SETTINGS, TICKRATE},
  state::State,
};

#[derive(Debug)]
pub struct JoyStickState {
  x: i32,
  y: i32,
  up: State,
  down: State,
  left: State,
  right: State,
  direction: Option<Direction>,
  last_event: Instant,
  tick_start: Instant,
  mouse_deltas: Vec<(f64, f64)>,
}

impl Default for JoyStickState {
  fn default() -> Self {
    Self {
      x: Default::default(),
      y: Default::default(),
      up: Default::default(),
      down: Default::default(),
      left: Default::default(),
      right: Default::default(),
      direction: Default::default(),
      last_event: Instant::now(),
      tick_start: Instant::now(),
      mouse_deltas: Default::default(),
    }
  }
}

impl JoyStickState {
  pub fn tilt(&mut self, vector: Vector) -> Vector {
    self.last_event = Instant::now();

    let sensitivity = LEFT_STICK_SENSITIVITY;

    self.x += vector.dx() * sensitivity;
    self.y += vector.dy() * sensitivity;

    let magnitude = ((self.x as f64).powi(2) + (self.y as f64).powi(2)).sqrt();
    if magnitude > MAX_STICK_TILT as f64 {
      let scale = MAX_STICK_TILT as f64 / magnitude;
      self.x = (self.x as f64 * scale).round() as i32;
      self.y = (self.y as f64 * scale).round() as i32;
    }

    Vector::new(self.x, self.y)
  }

  pub fn micro(&mut self, vector: Vector) -> Vector {
    const MOVEMENT_DEADZONE: f64 = 0.5; // Speed threshold to ignore jitter
    const BLEND: f64 = 0.2; // Smoothing factor (0.0 = none, 1.0 = slowest)

    let now = Instant::now();
    self.mouse_deltas.push((vector.dx() as f64, vector.dy() as f64));
    let elapsed = now.duration_since(self.tick_start);

    if elapsed >= TICKRATE {
      let (sum_dx, sum_dy): (f64, f64) = self
        .mouse_deltas
        .iter()
        .copied()
        .fold((0.0, 0.0), |acc, (dx, dy)| (acc.0 + dx, acc.1 + dy));

      let raw_speed = (sum_dx.powi(2) + sum_dy.powi(2)).sqrt();
      let sensitivity = SETTINGS.sensitivity();
      let scaled_speed = raw_speed * sensitivity;

      if scaled_speed < MOVEMENT_DEADZONE {
        self.mouse_deltas.clear();
        self.tick_start = now;
        self.last_event = now;
        self.x = 0;
        self.y = 0;
        return Vector::new(0, 0);
      }

      let angle = sum_dy.atan2(sum_dx);
      let clamped_speed = scaled_speed.clamp(1.0, 100.0);
      let normalized_speed = (clamped_speed - 1.0) / 99.0;

      let min_tilt = (MAX_STICK_TILT as f64) * 0.3;
      let max_tilt = (MAX_STICK_TILT as f64) * 1.0;
      let tilt = min_tilt + (max_tilt - min_tilt) * normalized_speed;

      let target_x = tilt * angle.cos();
      let target_y = tilt * angle.sin();

      self.x = ((1.0 - BLEND) * self.x as f64 + BLEND * target_x).round() as i32;
      self.y = ((1.0 - BLEND) * self.y as f64 + BLEND * target_y).round() as i32;

      self.mouse_deltas.clear();
      self.tick_start = now;
      self.last_event = now;
    }

    Vector::new(self.x, self.y)
  }

  pub fn update_direction(&mut self) {
    let up = matches!(self.up, State::Pressed | State::Held);
    let down = matches!(self.down, State::Pressed | State::Held);
    let left = matches!(self.left, State::Pressed | State::Held);
    let right = matches!(self.right, State::Pressed | State::Held);

    self.direction = match (up, down, left, right) {
      (true, false, true, false) => Some(Direction::NorthWest),
      (true, false, false, true) => Some(Direction::NorthEast),
      (true, false, false, false) => Some(Direction::North),
      (false, true, true, false) => Some(Direction::SouthWest),
      (false, true, false, true) => Some(Direction::SouthEast),
      (false, true, false, false) => Some(Direction::South),
      (false, false, true, false) => Some(Direction::West),
      (false, false, false, true) => Some(Direction::East),
      _ => None,
    };
  }

  pub fn recenter(&mut self) {
    *self = Self::default();
  }

  pub fn set_up(&mut self, up: State) {
    self.up = up;
  }

  pub fn set_down(&mut self, down: State) {
    self.down = down;
  }

  pub fn set_left(&mut self, left: State) {
    self.left = left;
  }

  pub fn set_right(&mut self, right: State) {
    self.right = right;
  }

  pub fn direction(&self) -> Option<Direction> {
    self.direction
  }

  pub fn x(&self) -> i32 {
    self.x
  }

  pub fn y(&self) -> i32 {
    self.y
  }

  pub fn last_event(&self) -> Instant {
    self.last_event
  }

  pub fn is_idle(&self, timeout: Duration) -> bool {
    let now = Instant::now();
    let elapsed = now.duration_since(self.last_event());
    elapsed > timeout && (self.x() != 0 || self.y() != 0)
  }

  pub fn handle_idle(&mut self) -> bool {
    if self.is_idle(MOUSE_IDLE_TIMEOUT) {
      self.recenter();
      return true;
    }
    false
  }
}
