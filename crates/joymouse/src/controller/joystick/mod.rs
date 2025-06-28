use evdev::{KeyCode, KeyEvent, RelativeAxisCode, RelativeAxisEvent};

use crate::controller::joystick::axis::{AxisError, JoyStickAxis};

mod axis;

#[derive(Debug)]
pub enum JoyStickError {
  Axis(AxisError),
  UnsupportedAxisCode(RelativeAxisCode),
  UnsupportedKeyCode(KeyCode),
}

impl From<AxisError> for JoyStickError {
  fn from(value: AxisError) -> Self {
    Self::Axis(value)
  }
}

#[derive(Debug)]
pub enum JoyStick {
  Left,
  Right,
}

impl TryFrom<RelativeAxisCode> for JoyStick {
  type Error = JoyStickError;

  fn try_from(value: RelativeAxisCode) -> Result<Self, Self::Error> {
    let joystick = match value {
      RelativeAxisCode::REL_X => Self::Right,
      RelativeAxisCode::REL_Y => Self::Right,
      other => return Err(JoyStickError::UnsupportedAxisCode(other)),
    };
    Ok(joystick)
  }
}

impl TryFrom<RelativeAxisEvent> for JoyStick {
  type Error = JoyStickError;

  fn try_from(value: RelativeAxisEvent) -> Result<Self, Self::Error> {
    Self::try_from(value.code())
  }
}

impl TryFrom<KeyCode> for JoyStick {
  type Error = JoyStickError;

  fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
    let joystick = match value {
      KeyCode::KEY_W => Self::Left,
      KeyCode::KEY_A => Self::Left,
      KeyCode::KEY_S => Self::Left,
      KeyCode::KEY_D => Self::Left,
      other => return Err(JoyStickError::UnsupportedKeyCode(other)),
    };
    Ok(joystick)
  }
}

impl TryFrom<KeyEvent> for JoyStick {
  type Error = JoyStickError;

  fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
    Self::try_from(value.code())
  }
}

#[derive(Debug)]
pub struct ControllerJoyStickEvent {
  joystick: JoyStick,
  axis: JoyStickAxis,
  value: i32,
}

impl ControllerJoyStickEvent {
  pub fn joystick(&self) -> &JoyStick {
    &self.joystick
  }

  pub fn axis(&self) -> &JoyStickAxis {
    &self.axis
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

impl TryFrom<RelativeAxisEvent> for ControllerJoyStickEvent {
  type Error = JoyStickError;

  fn try_from(value: RelativeAxisEvent) -> Result<Self, Self::Error> {
    let (code, value) = value.destructure();
    let joystick = JoyStick::try_from(code)?;
    let axis = JoyStickAxis::try_from(code)?;

    Ok(Self {
      joystick,
      axis,
      value,
    })
  }
}
