use crate::controller::{
  joystick::{JoyStick, axis::JoyStickAxis, direction::Direction, polarity::Polarity},
  settings::{MAX_STICK_TILT, MIN_STICK_TILT},
};

#[derive(Default, Debug, Copy, Clone)]
pub struct Vector {
  dx: i32,
  dy: i32,
}

impl Vector {
  pub fn new(dx: i32, dy: i32) -> Self {
    Self {
      dx: dx.clamp(MIN_STICK_TILT, MAX_STICK_TILT),
      dy: dy.clamp(MIN_STICK_TILT, MAX_STICK_TILT),
    }
  }

  pub fn flipped_y(&self) -> Self {
    Self::new(self.dx, -self.dy)
  }

  pub fn dx(&self) -> i32 {
    self.dx
  }

  pub fn dy(&self) -> i32 {
    self.dy
  }
}

impl std::ops::Mul<i32> for Vector {
  type Output = Vector;

  fn mul(self, rhs: i32) -> Self::Output {
    Vector::new(self.dx() * rhs, self.dy() * rhs)
  }
}

impl From<(&JoyStickAxis, Polarity, &JoyStick, Option<Direction>)> for Vector {
  fn from(value: (&JoyStickAxis, Polarity, &JoyStick, Option<Direction>)) -> Self {
    let (axis, polarity, joystick, direction) = value;
    match joystick {
      JoyStick::Left => direction.map(Vector::from).unwrap_or_default().flipped_y(),
      JoyStick::Right => {
        let delta = i32::from(polarity);
        match axis {
          JoyStickAxis::X => Vector::new(delta, 0),
          JoyStickAxis::Y => Vector::new(0, delta),
        }
      }
    }
  }
}

impl From<Direction> for Vector {
  fn from(direction: Direction) -> Self {
    match direction {
      Direction::North => Vector::new(0, 1),
      Direction::NorthEast => Vector::new(1, 1),
      Direction::East => Vector::new(1, 0),
      Direction::SouthEast => Vector::new(1, -1),
      Direction::South => Vector::new(0, -1),
      Direction::SouthWest => Vector::new(-1, -1),
      Direction::West => Vector::new(-1, 0),
      Direction::NorthWest => Vector::new(-1, 1),
    }
  }
}
