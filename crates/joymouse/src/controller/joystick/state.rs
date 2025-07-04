use std::time::{Duration, Instant};

use crate::controller::{
  joystick::{direction::Direction, vector::Vector},
  settings::{MAX_STICK_TILT, MIN_STICK_TILT, MOUSE_IDLE_TIMEOUT, SETTINGS, TICKRATE},
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

    let sensitivity = SETTINGS.sensitivity();

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
      let angle = sum_dy.atan2(sum_dx); // angle in radians
      let angle_degrees = angle.to_degrees();

      // Clamp speed to 1-100
      let clamped_speed = raw_speed.clamp(1.0, 100.0);
      let normalized_speed = (clamped_speed - 1.0) / 99.0; // Range: 0.0 - 1.0

      // Calculate tilt magnitude based on speed
      let min_tilt = (MAX_STICK_TILT as f64) * 0.3;
      let max_tilt = (MAX_STICK_TILT as f64) * 1.0;
      let tilt = min_tilt + (max_tilt - min_tilt) * normalized_speed;

      // Final vector calculation
      let x = tilt * angle.cos();
      let y = tilt * angle.sin(); // y shrinks when going up, so this is correct

      println!(
        "[micro] Commit: dx sum = {:.2}, dy sum = {:.2}, speed = {:.2}, angle = {:.2}°, clamped speed = {:.2}, tilt = {:.2}, x = {:.2}, y = {:.2}, events = {}",
        sum_dx,
        sum_dy,
        raw_speed,
        angle_degrees,
        clamped_speed,
        tilt,
        x,
        y,
        self.mouse_deltas.len()
      );

      self.mouse_deltas.clear();
      self.tick_start = now;

      let tilt_vector = Vector::new(x.round() as i32, y.round() as i32);
      return self.tilt(tilt_vector);
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
