use std::time::{Duration, Instant};

use crate::controller::{
  joystick::{direction::Direction, vector::Vector},
  settings::SETTINGS,
  state::State,
};

#[derive(Debug)]
pub struct JoyStickState {
  x: f64,
  y: f64,
  up: State,
  down: State,
  left: State,
  right: State,
  direction: Option<Direction>,
  last_event: Instant,
  tick_start: Instant,
  mouse_events: Vec<Vector>,
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
      mouse_events: Default::default(),
    }
  }
}

impl JoyStickState {
  pub fn tilt(&mut self, vector: Vector) -> Vector {
    self.last_event = Instant::now();

    let sensitivity = SETTINGS.left_stick_sensitivity();

    self.x += vector.dx() * sensitivity;
    self.y += vector.dy() * sensitivity;

    let magnitude = ((self.x).powi(2) + (self.y).powi(2)).sqrt();
    if magnitude > SETTINGS.max_stick_tilt() {
      let scale = SETTINGS.max_stick_tilt() / magnitude;
      self.x = (self.x * scale).round();
      self.y = (self.y * scale).round();
    }

    Vector::new(self.x, self.y)
  }

  pub fn micro(&mut self, vector: Vector) -> Vector {
    let now = Instant::now();
    let elapsed = now.duration_since(self.tick_start);

    self.mouse_events.push(vector);

    if elapsed >= SETTINGS.tickrate() {
      let (dx, dy) = self
        .mouse_events
        .iter()
        .copied()
        .fold((0.0, 0.0), |acc, vector| (acc.0 + vector.dx(), acc.1 + vector.dy()));

      let raw_speed = (dx.powi(2) + dy.powi(2)).sqrt();
      let sensitivity = SETTINGS.right_stick_sensitivity();
      let scaled_speed = raw_speed * sensitivity;
      let angle = dy.atan2(dx);
      let clamped_speed = scaled_speed.clamp(1.0, 100.0);
      let normalized_speed = (clamped_speed - 1.0) / 99.0;

      let min_tilt = SETTINGS.min_tilt_range();
      let max_tilt = SETTINGS.max_tilt_range();
      let tilt = min_tilt + (max_tilt - min_tilt) * normalized_speed;

      let target_x = tilt * angle.cos();
      let target_y = tilt * angle.sin();

      self.update_smoothed_position(target_x, target_y, SETTINGS.blend());

      self.mouse_events.clear();
      self.tick_start = now;
      self.last_event = now;
    }

    Vector::new(self.x, self.y)
  }

  fn update_smoothed_position(&mut self, target_x: f64, target_y: f64, blend: f64) {
    let target_mag = (target_x.powi(2) + target_y.powi(2)).sqrt();
    let min_tilt = SETTINGS.min_tilt_range();

    if target_mag < min_tilt {
      println!(
        "[update_smoothed_position] Suppressed update: target too small ({:.2} < {:.2})",
        target_mag, min_tilt
      );
      return;
    }

    let prev = Vector::new(self.x, self.y);

    let blended_x = (1.0 - blend) * self.x + blend * target_x;
    let blended_y = (1.0 - blend) * self.y + blend * target_y;

    let current = Vector::new(blended_x.round(), blended_y.round());

    let dot = prev.dx() * current.dx() + prev.dy() * current.dy();
    if dot < 0.0 {
      println!(
        "[update_smoothed_position] Reversal: ({:.0},{:.0}) → ({:.0},{:.0})",
        prev.dx(),
        prev.dy(),
        current.dx(),
        current.dy()
      );
    }

    let current_mag = (current.dx().powi(2) + current.dy().powi(2)).sqrt();
    if current_mag < 1.0 && (prev.dx() != 0.0 || prev.dy() != 0.0) {
      println!(
        "[update_smoothed_position] Sudden stop at ({:.0},{:.0})",
        current.dx(),
        current.dy()
      );
    }

    self.x = current.dx();
    self.y = current.dy();
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

  pub fn x(&self) -> f64 {
    self.x
  }

  pub fn y(&self) -> f64 {
    self.y
  }

  pub fn last_event(&self) -> Instant {
    self.last_event
  }

  pub fn is_idle(&self, timeout: Duration) -> bool {
    let now = Instant::now();
    let elapsed = now.duration_since(self.last_event());
    elapsed > timeout && (self.x() != 0.0 || self.y() != 0.0)
  }

  pub fn handle_idle(&mut self) -> bool {
    if self.is_idle(SETTINGS.mouse_idle_timeout()) {
      self.recenter();
      return true;
    }
    false
  }
}
