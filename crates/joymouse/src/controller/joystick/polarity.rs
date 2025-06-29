use evdev::KeyCode;

use crate::controller::{
  button::ControllerButton,
  joystick::{JoyStickError, axis::JoyStickAxis},
};

#[derive(Debug, Copy, Clone)]
pub enum Polarity {
  Positive(i32),
  Negative(i32),
}

impl Polarity {
  pub fn magnitude(&self) -> i32 {
    match self {
      Polarity::Positive(val) | Polarity::Negative(val) => val.abs(),
    }
  }

  pub fn sign(&self) -> i32 {
    match self {
      Polarity::Positive(_) => 1,
      Polarity::Negative(_) => -1,
    }
  }
}

impl From<Polarity> for i32 {
  fn from(value: Polarity) -> Self {
    match value {
      Polarity::Positive(strength) => strength,
      Polarity::Negative(strength) => strength,
    }
  }
}

impl TryFrom<(&JoyStickAxis, &ControllerButton, KeyCode)> for Polarity {
  type Error = JoyStickError;

  fn try_from(value: (&JoyStickAxis, &ControllerButton, KeyCode)) -> Result<Self, Self::Error> {
    let (axis, button, code) = value;

    match axis {
      JoyStickAxis::X => match button {
        ControllerButton::Starboard => Ok(Polarity::Positive(1)),
        ControllerButton::Port => Ok(Polarity::Negative(-1)),
        _ => Err(JoyStickError::UnsupportedKeyCode(code)),
      },
      JoyStickAxis::Y => match button {
        ControllerButton::Forward => Ok(Polarity::Positive(1)),
        ControllerButton::Backward => Ok(Polarity::Negative(-1)),
        _ => Err(JoyStickError::UnsupportedKeyCode(code)),
      },
    }
  }
}

impl TryFrom<i32> for Polarity {
  type Error = PolarityError;

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    if value > 0 {
      Ok(Self::Positive(value))
    } else if value < 0 {
      Ok(Self::Negative(value))
    } else {
      Err(PolarityError::InvalidPolarity(value))
    }
  }
}

#[derive(Debug)]
pub enum PolarityError {
  InvalidPolarity(i32),
}

impl std::fmt::Display for PolarityError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      PolarityError::InvalidPolarity(polarity) => {
        writeln!(f, "Invalid polarity: {}", polarity)
      }
    }
  }
}
