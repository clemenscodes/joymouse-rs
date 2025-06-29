mod error;

use evdev::{KeyCode, KeyEvent, RelativeAxisCode, RelativeAxisEvent};

use crate::controller::{
  button::ControllerButton,
  joystick::{JOYSTICK_KEYS, JoyStickKeys},
};

#[derive(Debug)]
pub enum JoyStickAxis {
  X,
  Y,
}

impl TryFrom<(JoyStickKeys, KeyCode)> for JoyStickAxis {
  type Error = AxisError;

  fn try_from((keys, code): (JoyStickKeys, KeyCode)) -> Result<Self, Self::Error> {
    if !keys.code_is_joystick_key(code) {
      return Err(AxisError::Unknown);
    }
    let axis = if keys.forward == code || keys.backward == code {
      Self::Y
    } else {
      Self::X
    };
    Ok(axis)
  }
}

impl TryFrom<RelativeAxisCode> for JoyStickAxis {
  type Error = AxisError;

  fn try_from(value: RelativeAxisCode) -> Result<Self, Self::Error> {
    let axis = match value {
      RelativeAxisCode::REL_X => Self::X,
      RelativeAxisCode::REL_Y => Self::Y,
      _ => return Err(AxisError::Unknown),
    };
    Ok(axis)
  }
}

impl TryFrom<RelativeAxisEvent> for JoyStickAxis {
  type Error = AxisError;

  fn try_from(value: RelativeAxisEvent) -> Result<Self, Self::Error> {
    Self::try_from(value.code())
  }
}

impl TryFrom<KeyCode> for JoyStickAxis {
  type Error = AxisError;

  fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
    if !JOYSTICK_KEYS.code_is_joystick_key(value) {
      return Err(AxisError::Unknown);
    }

    let button = ControllerButton::try_from(value)?;

    let axis = match button {
      ControllerButton::Forward => JoyStickAxis::Y,
      ControllerButton::Backward => JoyStickAxis::Y,
      ControllerButton::Starboard => JoyStickAxis::X,
      ControllerButton::Port => JoyStickAxis::X,
      _ => return Err(AxisError::Unknown),
    };

    Ok(axis)
  }
}

impl TryFrom<KeyEvent> for JoyStickAxis {
  type Error = AxisError;

  fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
    Self::try_from(value.code())
  }
}

pub use error::AxisError;
