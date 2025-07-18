use crate::{
  joystick::{direction::Direction, JoyStick},
  Axis, Polarity,
};

use settings::{MAX_STICK_TILT, MIN_STICK_TILT};

#[derive(Default, Debug, Copy, Clone)]
pub struct Vector {
  dx: f64,
  dy: f64,
}

impl Vector {
  pub fn new(dx: f64, dy: f64) -> Self {
    Self {
      dx: dx.clamp(MIN_STICK_TILT, MAX_STICK_TILT),
      dy: dy.clamp(MIN_STICK_TILT, MAX_STICK_TILT),
    }
  }

  pub fn flipped_y(&self) -> Self {
    Self::new(self.dx, -self.dy)
  }

  pub fn dx(&self) -> f64 {
    self.dx
  }

  pub fn dy(&self) -> f64 {
    self.dy
  }

  pub fn sum(vectors: &[Self]) -> Self {
    let (x, y) = vectors.iter().copied().fold((0.0, 0.0), |(dx, dy), v| (dx + v.dx(), dy + v.dy()));
    Self::new(x, y)
  }
}

impl std::ops::Mul<f64> for Vector {
  type Output = Vector;

  fn mul(self, rhs: f64) -> Self::Output {
    Vector::new(self.dx() * rhs, self.dy() * rhs)
  }
}

impl From<(&Axis, Polarity, &JoyStick, Option<Direction>)> for Vector {
  fn from(value: (&Axis, Polarity, &JoyStick, Option<Direction>)) -> Self {
    let (axis, polarity, joystick, direction) = value;
    match joystick {
      JoyStick::Left => direction.map(Vector::from).unwrap_or_default().flipped_y(),
      JoyStick::Right => {
        let delta = i32::from(polarity) as f64;
        match axis {
          Axis::X => Vector::new(delta, 0.0),
          Axis::Y => Vector::new(0.0, delta),
        }
      }
    }
  }
}

impl From<Direction> for Vector {
  fn from(direction: Direction) -> Self {
    match direction {
      Direction::North => Vector::new(0.0, 1.0),
      Direction::NorthEast => Vector::new(1.0, 1.0),
      Direction::East => Vector::new(1.0, 0.0),
      Direction::SouthEast => Vector::new(1.0, -1.0),
      Direction::South => Vector::new(0.0, -1.0),
      Direction::SouthWest => Vector::new(-1.0, -1.0),
      Direction::West => Vector::new(-1.0, 0.0),
      Direction::NorthWest => Vector::new(-1.0, 1.0),
    }
  }
}
