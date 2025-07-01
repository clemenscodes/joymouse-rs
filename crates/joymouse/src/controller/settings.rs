use crate::controller::button::ControllerButton;

use std::{collections::HashMap, sync::LazyLock, time::Duration};

use evdev::KeyCode;

#[derive(Debug)]
pub struct ControllerSettings {
  sensitivity: i32,
}

impl ControllerSettings {
  pub fn sensitivity(&self) -> i32 {
    self.sensitivity
  }
}

impl Default for ControllerSettings {
  fn default() -> Self {
    Self {
      sensitivity: RIGHT_STICK_SENSITIVITY,
    }
  }
}

pub const NAME: &str = "JoyMouse";
pub const VENDOR: u16 = 0x1234;
pub const PRODUCT: u16 = 0x5678;
pub const VERSION: u16 = 0x0100;
pub const MAX_STICK_TILT: i32 = 32767;
pub const MIN_STICK_TILT: i32 = -MAX_STICK_TILT - 1;
pub const DEADZONE: i32 = 0;
pub const NOISE_TOLERANCE: i32 = 0;
pub const TICKRATE: Duration = Duration::from_millis(16);
pub const LEFT_STICK_SENSITIVITY: i32 = 10000;
pub const RIGHT_STICK_SENSITIVITY: i32 = 250;
pub const MOUSE_IDLE_TIMEOUT: Duration = Duration::from_millis(20);

pub static SETTINGS: LazyLock<ControllerSettings> = LazyLock::new(ControllerSettings::default);

pub static CONTROLLER_KEY_MAP: LazyLock<HashMap<KeyCode, ControllerButton>> = LazyLock::new(|| {
  let mut map = HashMap::new();

  map.insert(KeyCode::KEY_SPACE, ControllerButton::South);
  map.insert(KeyCode::KEY_LEFTCTRL, ControllerButton::East);
  map.insert(KeyCode::KEY_F, ControllerButton::North);
  map.insert(KeyCode::KEY_C, ControllerButton::West);
  map.insert(KeyCode::KEY_UP, ControllerButton::Up);
  map.insert(KeyCode::KEY_LEFT, ControllerButton::Left);
  map.insert(KeyCode::KEY_DOWN, ControllerButton::Down);
  map.insert(KeyCode::KEY_RIGHT, ControllerButton::Right);
  map.insert(KeyCode::BTN_LEFT, ControllerButton::R1);
  map.insert(KeyCode::BTN_RIGHT, ControllerButton::L1);
  map.insert(KeyCode::KEY_Q, ControllerButton::L2);
  map.insert(KeyCode::KEY_X, ControllerButton::R2);
  map.insert(KeyCode::KEY_LEFTALT, ControllerButton::L3);
  map.insert(KeyCode::KEY_V, ControllerButton::R3);
  map.insert(KeyCode::KEY_TAB, ControllerButton::Select);
  map.insert(KeyCode::KEY_ENTER, ControllerButton::Start);
  map.insert(KeyCode::KEY_W, ControllerButton::Forward);
  map.insert(KeyCode::KEY_A, ControllerButton::Port);
  map.insert(KeyCode::KEY_S, ControllerButton::Backward);
  map.insert(KeyCode::KEY_D, ControllerButton::Starboard);

  for button in ControllerButton::all() {
    assert!(
      map.values().any(|b| b == button),
      "Missing mapping for ControllerButton::{:?}",
      button
    );
  }

  map
});

#[rustfmt::skip]
pub static KEYBOARD_BUTTON_MAP: LazyLock<HashMap<ControllerButton, KeyCode>> = LazyLock::new(|| 
  CONTROLLER_KEY_MAP.iter().map(|(k, v)| (*v, *k)).collect()
);
