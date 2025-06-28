mod axis;
mod event;
mod state;

use crate::controller::{
  Controller,
  button::{ButtonError, ControllerButton, KEYBOARD_BUTTON_MAP},
};

use std::sync::LazyLock;

use evdev::{AbsoluteAxisCode, EventType, InputEvent, KeyCode, KeyEvent, RelativeAxisCode, RelativeAxisEvent};

pub static JOYSTICK_KEYS: LazyLock<JoyStickKeys> = LazyLock::new(JoyStickKeys::default);

#[derive(Debug, Copy, Clone)]
pub struct JoyStickKeys {
  forward: KeyCode,
  backward: KeyCode,
  port: KeyCode,
  starboard: KeyCode,
}

impl Default for JoyStickKeys {
  fn default() -> Self {
    Self {
      forward: *KEYBOARD_BUTTON_MAP.get(&ControllerButton::Forward).unwrap(),
      backward: *KEYBOARD_BUTTON_MAP.get(&ControllerButton::Backward).unwrap(),
      port: *KEYBOARD_BUTTON_MAP.get(&ControllerButton::Port).unwrap(),
      starboard: *KEYBOARD_BUTTON_MAP.get(&ControllerButton::Starboard).unwrap(),
    }
  }
}

impl JoyStickKeys {
  pub fn code_is_joystick_key(&self, code: KeyCode) -> bool {
    code == self.forward || code == self.backward || code == self.port || code == self.starboard
  }
}

#[derive(Debug)]
pub enum JoyStickError {
  Axis(AxisError),
  UnsupportedAxisCode(RelativeAxisCode),
  UnsupportedKeyCode(KeyCode),
  Button(ButtonError),
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
    if !JOYSTICK_KEYS.code_is_joystick_key(value) {
      return Err(JoyStickError::UnsupportedKeyCode(value));
    }
    Ok(JoyStick::Left)
  }
}

impl TryFrom<KeyEvent> for JoyStick {
  type Error = JoyStickError;

  fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
    Self::try_from(value.code())
  }
}

impl Controller {
  pub fn handle_joystick_event(&mut self, event: ControllerJoyStickEvent) {
    println!("Handling controller joystick event: {:#?}", event);

    let code = AbsoluteAxisCode::from(&event);

    let delta = event.value();

    let value = match event.joystick() {
      JoyStick::Left => match event.axis() {
        axis::JoyStickAxis::X => self.left_stick.x(-delta),
        axis::JoyStickAxis::Y => self.left_stick.y(-delta),
      },
      JoyStick::Right => match event.axis() {
        axis::JoyStickAxis::X => self.right_stick.x(delta),
        axis::JoyStickAxis::Y => self.right_stick.y(delta),
      },
    };

    let virtual_event = InputEvent::new(EventType::ABSOLUTE.0, code.0, value);
    let events = vec![virtual_event];
    self.virtual_device.emit(&events).unwrap();
  }
}

pub use axis::AxisError;
pub use event::ControllerJoyStickEvent;
pub use state::JoyStickState;
