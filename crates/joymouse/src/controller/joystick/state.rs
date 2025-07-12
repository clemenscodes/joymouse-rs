use std::time::Instant;

use crate::controller::{
  joystick::{direction::Direction, motion::Motion, vector::Vector},
  settings::SETTINGS,
  state::State,
};

const MAX_MOTION_HISTORY: usize = 5;
const SPEED_CLAMP_MIN: f64 = 1.0;
const SPEED_CLAMP_MAX: f64 = 500.0;
const SPEED_NORMALIZER: f64 = SPEED_CLAMP_MAX - SPEED_CLAMP_MIN;
const DIAGONAL_BOOST: f64 = 1.41;
const ANGLE_SMOOTH_THRESHOLD: f64 = 0.5;
const SPEED_DELTA_THRESHOLD: f64 = 200.0;
const STABLE_MAG_LOWER_BOUND: f64 = 0.001;

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
    let sensitivity = SETTINGS.left_stick_sensitivity();
    self.last_event = Instant::now();
    self.x += vector.dx() * sensitivity;
    self.y += vector.dy() * sensitivity;
    self.clamp_tilt();
    self.vector()
  }

  pub fn micro(&mut self, vector: Vector) -> Vector {
    let now = Instant::now();
    let elapsed = now.duration_since(self.tick_start);
    self.mouse_events.push(vector);

    if self.mouse_events.len() >= 2 {
      let vector = Vector::sum(&self.mouse_events);
      let speed = speed(vector, SETTINGS.right_stick_sensitivity());

      self.motion_history.push(speed);
      if self.motion_history.len() > MAX_MOTION_HISTORY {
        self.motion_history.remove(0);
      }

      let avg = average(&self.motion_history);
      let motion = Motion::resolve(avg);
      self.motion = Motion::compare(avg, self.motion, motion);

      if self.motion == Motion::Flick {
        return self.commit(now);
      }

      if self.motion == Motion::Micro {
        self.last_event = now;
      }
    }

    if elapsed >= SETTINGS.tickrate() {
      return self.commit(now);
    }

    self.vector()
  }

  fn commit(&mut self, now: Instant) -> Vector {
    self.tick_start = now;
    self.last_event = now;

    if self.mouse_events.len() < 2 {
      return self.vector();
    }

    let vector = Vector::sum(&self.mouse_events);
    let angle = vector.dy().atan2(vector.dx());
    let speed = speed(vector, SETTINGS.right_stick_sensitivity());
    let min = SETTINGS.min_tilt_range();
    let max = SETTINGS.max_tilt_range();
    let tilt = min + (max - min) * speed;
    let boost = if vector.dx().abs() > 0.0 && vector.dy().abs() > 0.0 {
      DIAGONAL_BOOST
    } else {
      1.0
    };
    let x = tilt * angle.cos() * boost;
    let y = tilt * angle.sin() * boost;

    self.update_smoothed_position(Vector::new(x, y), SETTINGS.blend());
    self.mouse_events.clear();
    self.vector()
  }

  fn update_smoothed_position(&mut self, vector: Vector, blend: f64) {
    let prev = self.vector();
    let max_tilt = SETTINGS.max_stick_tilt();
    let x = blend_value(prev.dx(), vector.dx(), blend);
    let y = blend_value(prev.dy(), vector.dy(), blend);
    let angle = compute_smoothed_angle(x, y, self.angle);

    self.angle = Some(angle);

    let mag = magnitude(Vector::new(x, y));
    let last_mag = magnitude(prev);
    let delta = (mag - last_mag).abs();

    let mag = if delta < SPEED_DELTA_THRESHOLD {
      last_mag
    } else {
      mag
    };

    let mag = if mag < SETTINGS.min_tilt_range() && mag > STABLE_MAG_LOWER_BOUND {
      SETTINGS.min_tilt_range()
    } else {
      mag
    };

    let radians = angle.to_radians();
    let mut x = mag * radians.cos();
    let mut y = mag * radians.sin();

    let length = magnitude(Vector::new(x, y));
    if length > max_tilt {
      let scale = max_tilt / length;
      x *= scale;
      y *= scale;
    }

    self.x = x;
    self.y = y;
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

  pub fn handle_idle(&mut self) -> bool {
    if self.is_idle() {
      self.recenter();
      return true;
    }
    false
  }

  fn is_idle(&self) -> bool {
    let now = Instant::now();
    let elapsed = now.duration_since(self.last_event);
    elapsed > SETTINGS.mouse_idle_timeout() && (self.x() != 0.0 || self.y() != 0.0)
  }

  fn recenter(&mut self) {
    *self = Self::default();
  }

  fn vector(&self) -> Vector {
    Vector::new(self.x, self.y)
  }

  fn clamp_tilt(&mut self) {
    let mag = magnitude(self.vector());
    let max = SETTINGS.max_stick_tilt();
    if mag > max {
      let scale = max / mag;
      self.x = (self.x * scale).round();
      self.y = (self.y * scale).round();
    }
  }
}

fn magnitude(vector: Vector) -> f64 {
  (vector.dx().powi(2) + vector.dy().powi(2)).sqrt()
}

fn speed(vector: Vector, sensitivity: f64) -> f64 {
  let speed = magnitude(vector);
  let scaled = speed * sensitivity;
  let clamped = scaled.clamp(SPEED_CLAMP_MIN, SPEED_CLAMP_MAX);
  (clamped - SPEED_CLAMP_MIN) / SPEED_NORMALIZER
}

fn average(values: &[f64]) -> f64 {
  values.iter().copied().sum::<f64>() / values.len() as f64
}

fn blend_value(prev: f64, new: f64, blend: f64) -> f64 {
  (1.0 - blend) * prev + blend * new
}

fn compute_smoothed_angle(x: f64, y: f64, prev: Option<f64>) -> f64 {
  let angle = y.atan2(x).to_degrees();
  match prev {
    Some(prev) => {
      let delta = ((angle - prev + 180.0) % 360.0) - 180.0;
      if delta.abs() < ANGLE_SMOOTH_THRESHOLD {
        prev
      } else {
        angle
      }
    }
    None => angle,
  }
}
