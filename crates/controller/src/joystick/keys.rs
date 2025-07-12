use std::sync::LazyLock;

use crate::{button::ControllerButton, settings::KEYBOARD_BUTTON_MAP};

use evdev::KeyCode;

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

  pub fn forward(&self) -> KeyCode {
    self.forward
  }

  pub fn backward(&self) -> KeyCode {
    self.backward
  }

  pub fn port(&self) -> KeyCode {
    self.port
  }

  pub fn starboard(&self) -> KeyCode {
    self.starboard
  }
}
