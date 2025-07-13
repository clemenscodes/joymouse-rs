use std::sync::LazyLock;

use crate::{button::ControllerButton, keys::CONTROLLER_KEY_MAP};

use evdev::KeyCode;

pub static JOYSTICK_KEYS: LazyLock<JoyStickKeys> = LazyLock::new(JoyStickKeys::default);

#[derive(Debug, Clone)]
pub struct JoyStickKeys {
  forward: Vec<KeyCode>,
  backward: Vec<KeyCode>,
  port: Vec<KeyCode>,
  starboard: Vec<KeyCode>,
}

impl Default for JoyStickKeys {
  fn default() -> Self {
    Self {
      forward: CONTROLLER_KEY_MAP.get(&ControllerButton::Forward).cloned().unwrap_or_default(),
      backward: CONTROLLER_KEY_MAP.get(&ControllerButton::Backward).cloned().unwrap_or_default(),
      port: CONTROLLER_KEY_MAP.get(&ControllerButton::Port).cloned().unwrap_or_default(),
      starboard: CONTROLLER_KEY_MAP.get(&ControllerButton::Starboard).cloned().unwrap_or_default(),
    }
  }
}

impl JoyStickKeys {
  pub fn code_is_joystick_key(&self, code: KeyCode) -> bool {
    self.forward.contains(&code)
      || self.backward.contains(&code)
      || self.port.contains(&code)
      || self.starboard.contains(&code)
  }

  pub fn forward(&self) -> &[KeyCode] {
    &self.forward
  }

  pub fn backward(&self) -> &[KeyCode] {
    &self.backward
  }

  pub fn port(&self) -> &[KeyCode] {
    &self.port
  }

  pub fn starboard(&self) -> &[KeyCode] {
    &self.starboard
  }
}
