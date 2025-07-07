use crate::controller::button::ControllerButton;
use evdev::KeyCode;
use std::{collections::HashMap, sync::LazyLock, time::Duration};

#[derive(Debug)]
pub struct ControllerSettings {
  name: &'static str,
  vendor: u16,
  product: u16,
  version: u16,
  max_stick_tilt: f64,
  min_stick_tilt: f64,
  deadzone: f64,
  noise_tolerance: f64,
  tickrate: Duration,
  left_stick_sensitivity: f64,
  right_stick_sensitivity: f64,
  minimum_tilt: f64,
  maximum_tilt: f64,
  blend: f64,
  mouse_idle_timeout: Duration,
  max_tilt_range: f64,
  min_tilt_range: f64,
}

impl ControllerSettings {
  pub const fn name(&self) -> &'static str {
    self.name
  }

  pub const fn vendor(&self) -> u16 {
    self.vendor
  }

  pub const fn product(&self) -> u16 {
    self.product
  }

  pub const fn version(&self) -> u16 {
    self.version
  }

  pub const fn max_stick_tilt(&self) -> f64 {
    self.max_stick_tilt
  }

  pub const fn min_stick_tilt(&self) -> f64 {
    self.min_stick_tilt
  }

  pub const fn deadzone(&self) -> f64 {
    self.deadzone
  }

  pub const fn noise_tolerance(&self) -> f64 {
    self.noise_tolerance
  }

  pub const fn tickrate(&self) -> Duration {
    self.tickrate
  }

  pub const fn left_stick_sensitivity(&self) -> f64 {
    self.left_stick_sensitivity
  }

  pub const fn right_stick_sensitivity(&self) -> f64 {
    self.right_stick_sensitivity
  }

  pub const fn blend(&self) -> f64 {
    self.blend
  }

  pub const fn mouse_idle_timeout(&self) -> Duration {
    self.mouse_idle_timeout
  }

  pub fn max_tilt_range(&self) -> f64 {
    self.max_tilt_range
  }

  pub fn min_tilt_range(&self) -> f64 {
    self.min_tilt_range
  }

  pub const fn minimum_tilt(&self) -> f64 {
    self.minimum_tilt
  }

  pub const fn maximum_tilt(&self) -> f64 {
    self.maximum_tilt
  }
}

impl Default for ControllerSettings {
  fn default() -> Self {
    let name = "JoyMouse";
    let vendor = 0x1234;
    let product = 0x5678;
    let version = 0x0100;
    let deadzone = 0.0;
    let noise_tolerance = 0.0;
    let tickrate = Duration::from_millis(16);
    let mouse_idle_timeout = Duration::from_millis(120);
    let left_stick_sensitivity = 10000.0;
    let right_stick_sensitivity = 10.0;
    let max_stick_tilt = 32767.0;
    let min_stick_tilt = -32768.0;
    let minimum_tilt = 0.4;
    let maximum_tilt = 1.0;
    let blend = 0.2;
    let max_tilt_range = max_stick_tilt * maximum_tilt;
    let min_tilt_range = max_stick_tilt * minimum_tilt;
    Self {
      name,
      vendor,
      product,
      version,
      max_stick_tilt,
      min_stick_tilt,
      deadzone,
      noise_tolerance,
      tickrate,
      left_stick_sensitivity,
      right_stick_sensitivity,
      minimum_tilt,
      maximum_tilt,
      blend,
      mouse_idle_timeout,
      max_tilt_range,
      min_tilt_range,
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
