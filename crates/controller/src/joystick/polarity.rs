use evdev::KeyCode;

use crate::{
  button::ControllerButton,
  joystick::{JoyStickError, axis::JoyStickAxis},
};

#[derive(Debug, Copy, Clone)]
pub enum Polarity {
  Positive(f64),
  Negative(f64),
}

impl Polarity {
  pub fn magnitude(&self) -> f64 {
    match self {
      Polarity::Positive(val) | Polarity::Negative(val) => val.abs(),
    }
  }

  pub fn sign(&self) -> f64 {
    match self {
      Polarity::Positive(_) => 1.0,
      Polarity::Negative(_) => -1.0,
    }
  }
}

impl From<Polarity> for f64 {
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
        ControllerButton::Starboard => Ok(Polarity::Positive(1.0)),
        ControllerButton::Port => Ok(Polarity::Negative(-1.0)),
        _ => Err(JoyStickError::UnsupportedKeyCode(code)),
      },
      JoyStickAxis::Y => match button {
        ControllerButton::Forward => Ok(Polarity::Positive(1.0)),
        ControllerButton::Backward => Ok(Polarity::Negative(-1.0)),
        _ => Err(JoyStickError::UnsupportedKeyCode(code)),
      },
    }
  }
}

impl TryFrom<f64> for Polarity {
  type Error = PolarityError;

  fn try_from(value: f64) -> Result<Self, Self::Error> {
    if value > 0.0 {
      Ok(Self::Positive(value))
    } else if value < 0.0 {
      Ok(Self::Negative(value))
    } else {
      Err(PolarityError::InvalidPolarity(value))
    }
  }
}

#[derive(Debug)]
pub enum PolarityError {
  InvalidPolarity(f64),
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
