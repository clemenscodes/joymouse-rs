use std::time::Instant;

use crate::{
  joystick::{direction::Direction, motion::Motion, vector::Vector},
  state::State,
};

use settings::{LEFT_STICK_SENSITIVITY, MAX_STICK_TILT, SETTINGS};

#[derive(Debug)]
pub struct JoyStickState {
  x: f64,
  y: f64,
  up: State,
  down: State,
  left: State,
  right: State,
  direction: Option<Direction>,
  motion: Motion,
  motion_history: Vec<f64>,
  angle: Option<f64>,
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
      motion: Default::default(),
      motion_history: Default::default(),
      angle: Default::default(),
      last_event: Instant::now(),
      tick_start: Instant::now(),
      mouse_events: Default::default(),
    }
  }
}

impl JoyStickState {
  pub fn tilt(&mut self, vector: Vector) -> Vector {
    self.last_event = Instant::now();
    self.x += vector.dx() * LEFT_STICK_SENSITIVITY;
    self.y += vector.dy() * LEFT_STICK_SENSITIVITY;
    self.clamp_position(MAX_STICK_TILT);
    self.vector()
  }

  pub fn micro(&mut self, vector: Vector) -> Vector {
    let now = Instant::now();
    self.mouse_events.push(vector);

    if self.mouse_events.len() >= 2 {
      let speed = self.compute_speed();
      self.motion = self.motion_from_speed(speed);

      match self.motion {
        Motion::Flick => {
          return self.commit(now);
        }
        Motion::Micro => {
          self.last_event = now;
        }
        _ => (),
      }
    }

    if now.duration_since(self.tick_start) >= SETTINGS.tickrate() {
      return self.commit(now);
    }

    self.vector()
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

  pub fn vector(&self) -> Vector {
    Vector::new(self.x, self.y)
  }

  pub fn is_idle(&self, left_stick_direction: Option<Direction>) -> bool {
    let now = Instant::now();
    let timeout = if self.motion == Motion::Micro && left_stick_direction.is_some() {
      SETTINGS.mouse_idle_timeout() * 5
    } else {
      SETTINGS.mouse_idle_timeout()
    };
    now.duration_since(self.last_event()) > timeout && !self.is_centered()
  }

  pub fn is_centered(&self) -> bool {
    self.x == 0.0 && self.y == 0.0
  }

  pub fn handle_idle(&mut self, left_stick_direction: Option<Direction>) -> bool {
    if self.is_idle(left_stick_direction) {
      self.recenter();
      return true;
    }
    false
  }

  fn last_event(&self) -> Instant {
    self.last_event
  }

  fn commit(&mut self, now: Instant) -> Vector {
    self.tick_start = now;
    self.last_event = now;
    if self.mouse_events.len() < 2 {
      return self.vector();
    }
    let vector = Vector::sum(&self.mouse_events);
    let tilt = if self.motion == Motion::Flick {
      SETTINGS.max_tilt_range()
    } else {
      let normalized_speed = self.calculate_normalized_speed(&vector);
      let min = SETTINGS.min_tilt_range();
      let max = SETTINGS.max_tilt_range();
      min + (max - min) * normalized_speed
    };
    let vector = self.compute_tilt_vector(vector, tilt);
    self.update_smoothed_position(vector, SETTINGS.blend());
    self.mouse_events.clear();
    self.vector()
  }

  fn compute_tilt_vector(&self, raw: Vector, tilt: f64) -> Vector {
    let boost = if raw.dx().abs() > 0.0 && raw.dy().abs() > 0.0 {
      SETTINGS.diagonal_boost()
    } else {
      1.0
    };
    let angle = raw.dy().atan2(raw.dx());
    let x = tilt * angle.cos() * boost;
    let y = tilt * angle.sin() * boost;
    Vector::new(x, y)
  }

  fn update_smoothed_position(&mut self, target: Vector, blend: f64) {
    let prev = self.vector();
    let min_tilt = SETTINGS.min_tilt_range();
    let x = (1.0 - blend) * prev.dx() + blend * target.dx();
    let y = (1.0 - blend) * prev.dy() + blend * target.dy();
    let vector = Vector::new(x, y);
    let magnitude = self.smooth_magnitude(prev, vector);
    let adjusted_mag = if magnitude < min_tilt && magnitude > 0.001 {
      min_tilt
    } else {
      magnitude
    };
    let angle = self.update_angle(vector);
    let final_x = adjusted_mag * angle.to_radians().cos();
    let final_y = adjusted_mag * angle.to_radians().sin();
    let vector = Vector::new(final_x, final_y);
    let vector = self.clamp_vector(vector, MAX_STICK_TILT);
    self.x = vector.dx();
    self.y = vector.dy();
  }

  fn clamp_vector(&self, v: Vector, max: f64) -> Vector {
    let length = (v.dx().powi(2) + v.dy().powi(2)).sqrt();
    if length > max {
      let scale = max / length;
      Vector::new(v.dx() * scale, v.dy() * scale)
    } else {
      v
    }
  }

  fn clamp_position(&mut self, max: f64) {
    let mag = (self.x.powi(2) + self.y.powi(2)).sqrt();
    if mag > max {
      let scale = max / mag;
      self.x = (self.x * scale).round();
      self.y = (self.y * scale).round();
    }
  }

  fn compute_speed(&mut self) -> f64 {
    let vector = Vector::sum(&self.mouse_events);
    let normalized = self.calculate_normalized_speed(&vector);

    self.motion_history.push(normalized);
    if self.motion_history.len() > 5 {
      self.motion_history.remove(0);
    }

    self.motion_history.iter().copied().sum::<f64>() / self.motion_history.len() as f64
  }

  fn calculate_normalized_speed(&self, vector: &Vector) -> f64 {
    let speed = (vector.dx().powi(2) + vector.dy().powi(2)).sqrt() * SETTINGS.sensitivity();
    let min = SETTINGS.min_speed_clamp();
    let max = SETTINGS.max_speed_clamp();
    let clamped = speed.clamp(min, max);
    (clamped - min) / (max - 1.0)
  }

  fn motion_from_speed(&self, avg: f64) -> Motion {
    let new_motion = Motion::from(avg);

    match (self.motion, new_motion) {
      (Motion::Macro, Motion::Micro) if avg > SETTINGS.motion_threshold_micro_macro_recover() => {
        Motion::Macro
      }
      (Motion::Micro, Motion::Macro) if avg < SETTINGS.motion_threshold_macro_micro() => {
        Motion::Micro
      }
      (_, updated) => updated,
    }
  }

  fn smooth_magnitude(&self, prev: Vector, new: Vector) -> f64 {
    let mag = (new.dx().powi(2) + new.dy().powi(2)).sqrt();
    let prev_mag = (prev.dx().powi(2) + prev.dy().powi(2)).sqrt();
    let delta = (mag - prev_mag).abs();
    if delta < SETTINGS.speed_stabilize_threshold() {
      prev_mag
    } else {
      mag
    }
  }

  fn update_angle(&mut self, v: Vector) -> f64 {
    let angle = v.dy().atan2(v.dx()).to_degrees();
    self.angle = match self.angle {
      Some(prev) => {
        let delta = ((angle - prev + 180.0) % 360.0) - 180.0;
        if delta.abs() < SETTINGS.angle_delta_limit() {
          Some(prev)
        } else {
          Some(angle)
        }
      }
      None => Some(angle),
    };
    self.angle.unwrap()
  }
}
