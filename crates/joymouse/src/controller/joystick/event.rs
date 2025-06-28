use evdev::{AbsoluteAxisCode, KeyEvent, RelativeAxisEvent};

use crate::controller::{
  button::{ButtonError, ControllerButton},
  joystick::{JOYSTICK_KEYS, JoyStick, JoyStickError, axis::JoyStickAxis},
};

#[derive(Debug)]
pub struct ControllerJoyStickEvent {
  joystick: JoyStick,
  axis: JoyStickAxis,
  value: i32,
}

impl ControllerJoyStickEvent {
  pub fn new(joystick: JoyStick, axis: JoyStickAxis, value: i32) -> Self {
    Self {
      joystick,
      axis,
      value,
    }
  }

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

impl From<ButtonError> for JoyStickError {
  fn from(value: ButtonError) -> Self {
    Self::Button(value)
  }
}

impl TryFrom<KeyEvent> for ControllerJoyStickEvent {
  type Error = JoyStickError;

  fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
    let code = value.code();
    let joystick = JoyStick::Left;
    let axis = JoyStickAxis::try_from((*JOYSTICK_KEYS, code))?;
    let button = ControllerButton::try_from(code)?;
    let value = match axis {
      JoyStickAxis::X => match button {
        ControllerButton::Starboard => 1,
        ControllerButton::Port => -1,
        _ => return Err(JoyStickError::UnsupportedKeyCode(code)),
      },
      JoyStickAxis::Y => match button {
        ControllerButton::Forward => 1,
        ControllerButton::Backward => -1,
        _ => return Err(JoyStickError::UnsupportedKeyCode(code)),
      },
    };
    Ok(Self::new(joystick, axis, value))
  }
}

impl TryFrom<RelativeAxisEvent> for ControllerJoyStickEvent {
  type Error = JoyStickError;

  fn try_from(value: RelativeAxisEvent) -> Result<Self, Self::Error> {
    let (code, value) = value.destructure();
    let joystick = JoyStick::try_from(code)?;
    let axis = JoyStickAxis::try_from(code)?;
    Ok(Self::new(joystick, axis, value))
  }
}

impl From<&ControllerJoyStickEvent> for AbsoluteAxisCode {
  fn from(value: &ControllerJoyStickEvent) -> Self {
    match value.joystick() {
      JoyStick::Left => match value.axis() {
        JoyStickAxis::X => Self::ABS_X,
        JoyStickAxis::Y => Self::ABS_Y,
      },
      JoyStick::Right => match value.axis() {
        JoyStickAxis::X => Self::ABS_RX,
        JoyStickAxis::Y => Self::ABS_RY,
      },
    }
  }
}
