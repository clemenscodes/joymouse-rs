use crate::CONTROLLER_KEY_MAP;

use controller::ControllerButton;

use io::Key;
use std::sync::LazyLock;

pub static JOYSTICK_KEYS: LazyLock<JoyStickKeys> = LazyLock::new(JoyStickKeys::default);

#[derive(Debug, Clone)]
pub struct JoyStickKeys {
  forward: Vec<Key>,
  backward: Vec<Key>,
  port: Vec<Key>,
  starboard: Vec<Key>,
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
  pub fn key_is_joystick_key(&self, key: Key) -> bool {
    self.forward.contains(&key)
      || self.backward.contains(&key)
      || self.port.contains(&key)
      || self.starboard.contains(&key)
  }

  pub fn forward(&self) -> &[Key] {
    &self.forward
  }

  pub fn backward(&self) -> &[Key] {
    &self.backward
  }

  pub fn port(&self) -> &[Key] {
    &self.port
  }

  pub fn starboard(&self) -> &[Key] {
    &self.starboard
  }
}
