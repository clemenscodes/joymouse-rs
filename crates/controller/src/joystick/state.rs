use std::time::Instant;

use crate::{
  joystick::{direction::Direction, motion::Motion, vector::Vector},
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

    let sensitivity = SETTINGS.left_stick_sensitivity();

    self.x += vector.dx() * sensitivity;
    self.y += vector.dy() * sensitivity;

    let magnitude = (self.x.powi(2) + self.y.powi(2)).sqrt();
    if magnitude > SETTINGS.max_stick_tilt() {
      let scale = SETTINGS.max_stick_tilt() / magnitude;
      self.x = (self.x * scale).round();
      self.y = (self.y * scale).round();
    }

    self.vector()
  }

  pub fn micro(&mut self, vector: Vector) -> Vector {
    let now = Instant::now();
    let elapsed = now.duration_since(self.tick_start);

    self.mouse_events.push(vector);

    if self.mouse_events.len() >= 2 {
      let vector = Vector::sum(&self.mouse_events);
      let raw_speed = (vector.dx().powi(2) + vector.dy().powi(2)).sqrt();
      let scaled_speed = raw_speed * SETTINGS.right_stick_sensitivity();
      let clamped_speed = scaled_speed.clamp(1.0, 500.0);
      let normalized_speed = (clamped_speed - 1.0) / 499.0;

      self.motion_history.push(normalized_speed);

      if self.motion_history.len() > 5 {
        self.motion_history.remove(0);
      }

      let avg_speed: f64 =
        self.motion_history.iter().copied().sum::<f64>() / self.motion_history.len() as f64;

      let motion = match avg_speed {
        s if s >= 0.5 => Motion::Flick,
        s if s >= 0.025 => Motion::Macro,
        _ => Motion::Micro,
      };

      self.motion = match (self.motion, motion) {
        (Motion::Macro, Motion::Micro) if avg_speed > 0.01 => Motion::Macro,
        (Motion::Micro, Motion::Macro) if avg_speed < 0.03 => Motion::Micro,
        (_, motion) => motion,
      };

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

    Vector::new(self.x, self.y)
  }

  fn commit(&mut self, now: Instant) -> Vector {
    self.tick_start = now;
    self.last_event = now;

    if self.mouse_events.len() < 2 {
      return Vector::new(self.x, self.y);
    }

    let vector = Vector::sum(&self.mouse_events);
    let raw_speed = (vector.dx().powi(2) + vector.dy().powi(2)).sqrt();
    let sensitivity = SETTINGS.right_stick_sensitivity();
    let scaled_speed = raw_speed * sensitivity;
    let clamped_speed = scaled_speed.clamp(1.0, 500.0);
    let normalized_speed = (clamped_speed - 1.0) / 499.0;
    let min_tilt = SETTINGS.min_tilt_range();
    let max_tilt = SETTINGS.max_tilt_range();
    let tilt = min_tilt + (max_tilt - min_tilt) * normalized_speed;

    let diagonal_boost = if vector.dx().abs() > 0.0 && vector.dy().abs() > 0.0 {
      1.41
    } else {
      1.0
    };

    let angle = vector.dy().atan2(vector.dx());
    let x = tilt * angle.cos() * diagonal_boost;
    let y = tilt * angle.sin() * diagonal_boost;

    let vector = Vector::new(x, y);

    self.update_smoothed_position(vector, SETTINGS.blend());

    self.mouse_events.clear();

    Vector::new(self.x, self.y)
  }

  fn update_smoothed_position(&mut self, vector: Vector, blend: f64) {
    let prev = self.vector();
    let min_tilt = SETTINGS.min_tilt_range();
    let max_tilt = SETTINGS.max_stick_tilt();

    let x = (1.0 - blend) * prev.dx() + blend * vector.dx();
    let y = (1.0 - blend) * prev.dy() + blend * vector.dy();

    let angle = y.atan2(x).to_degrees();

    let angle = match self.angle {
      Some(prev_angle) => {
        let delta = ((angle - prev_angle + 180.0) % 360.0) - 180.0;
        if delta.abs() < 0.5 {
          prev_angle
        } else {
          self.angle = Some(angle);
          angle
        }
      }
      None => {
        self.angle = Some(angle);
        angle
      }
    };

    let mag = (x.powi(2) + y.powi(2)).sqrt();
    let angle_rad = angle.to_radians();

    let last_mag = (prev.dx().powi(2) + prev.dy().powi(2)).sqrt();
    let speed_delta = (mag - last_mag).abs();

    let stable_mag = if speed_delta < 200.0 {
      last_mag
    } else {
      mag
    };

    let adjusted_mag = if stable_mag < min_tilt && stable_mag > 0.001 {
      min_tilt
    } else {
      stable_mag
    };

    let mut final_x = adjusted_mag * angle_rad.cos();
    let mut final_y = adjusted_mag * angle_rad.sin();

    let length = (final_x.powi(2) + final_y.powi(2)).sqrt();

    if length > max_tilt {
      let scale = max_tilt / length;
      final_x *= scale;
      final_y *= scale;
    }

    self.x = final_x;
    self.y = final_y;
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

  pub fn vector(&self) -> Vector {
    Vector::new(self.x, self.y)
  }

  pub fn last_event(&self) -> Instant {
    self.last_event
  }

  pub fn is_idle(&self, left_stick_direction: Option<Direction>) -> bool {
    let now = Instant::now();
    let elapsed = now.duration_since(self.last_event());

    let timeout = if (self.motion == Motion::Micro) && left_stick_direction.is_some() {
      SETTINGS.mouse_idle_timeout() * 5
    } else {
      SETTINGS.mouse_idle_timeout()
    };
    elapsed > timeout && (self.x() != 0.0 || self.y() != 0.0)
  }

  pub fn handle_idle(&mut self, left_stick_direction: Option<Direction>) -> bool {
    if self.is_idle(left_stick_direction) {
      self.recenter();
      return true;
    }
    false
  }
}
