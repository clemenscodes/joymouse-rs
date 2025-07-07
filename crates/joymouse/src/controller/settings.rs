use crate::controller::button::ControllerButton;
use evdev::KeyCode;
use std::{collections::HashMap, sync::LazyLock, time::Duration};

#[derive(Debug)]
pub struct ControllerSettings {
  name: &'static str,
  vendor: u16,
  product: u16,
  version: u16,
  max_stick_tilt: i32,
  min_stick_tilt: i32,
  deadzone: i32,
  noise_tolerance: i32,
  tickrate: Duration,
  left_stick_sensitivity: i32,
  right_stick_sensitivity: f64,
  minimum_tilt: f64,
  maximum_tilt: f64,
  mouse_idle_timeout: Duration,
}

impl ControllerSettings {
  pub fn name(&self) -> &'static str {
    self.name
  }

  pub fn vendor(&self) -> u16 {
    self.vendor
  }

  pub fn product(&self) -> u16 {
    self.product
  }

  pub fn version(&self) -> u16 {
    self.version
  }

  pub fn max_stick_tilt(&self) -> i32 {
    self.max_stick_tilt
  }

  pub fn min_stick_tilt(&self) -> i32 {
    self.min_stick_tilt
  }

  pub fn deadzone(&self) -> i32 {
    self.deadzone
  }

  pub fn noise_tolerance(&self) -> i32 {
    self.noise_tolerance
  }

  pub fn tickrate(&self) -> Duration {
    self.tickrate
  }

  pub fn left_stick_sensitivity(&self) -> i32 {
    self.left_stick_sensitivity
  }

  pub fn right_stick_sensitivity(&self) -> f64 {
    self.right_stick_sensitivity
  }

  pub fn minimum_tilt(&self) -> f64 {
    self.minimum_tilt
  }

  pub fn maximum_tilt(&self) -> f64 {
    self.maximum_tilt
  }

  pub fn mouse_idle_timeout(&self) -> Duration {
    self.mouse_idle_timeout
  }
}

impl Default for ControllerSettings {
  fn default() -> Self {
    Self {
      name: "JoyMouse",
      vendor: 0x1234,
      product: 0x5678,
      version: 0x0100,
      max_stick_tilt: 32767,
      min_stick_tilt: -32768,
      deadzone: 0,
      noise_tolerance: 0,
      tickrate: Duration::from_millis(16),
      left_stick_sensitivity: 10000,
      right_stick_sensitivity: 10.0,
      minimum_tilt: 0.40,
      maximum_tilt: 1.0,
      mouse_idle_timeout: Duration::from_millis(120),
    }
  }
}

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
