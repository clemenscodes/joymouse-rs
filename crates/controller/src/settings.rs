use crate::button::ControllerButton;
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
    let mouse_idle_timeout = tickrate * 4;
    let left_stick_sensitivity = 10000.0;
    let right_stick_sensitivity = 7.0;
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
      blend,
      mouse_idle_timeout,
      max_tilt_range,
      min_tilt_range,
    }
  }
}

pub static SETTINGS: LazyLock<ControllerSettings> = LazyLock::new(ControllerSettings::default);

#[rustfmt::skip]
pub static CONTROLLER_KEY_MAP: LazyLock<HashMap<ControllerButton, Vec<KeyCode>>> = LazyLock::new(|| {
    use ControllerButton::*;

    let mut map = HashMap::new();

    map.insert(South, vec![KeyCode::KEY_SPACE]);
    map.insert(East, vec![KeyCode::KEY_LEFTCTRL]);
    map.insert(North, vec![KeyCode::KEY_F]);
    map.insert(West, vec![KeyCode::KEY_C, KeyCode::BTN_SIDE]);

    map.insert(Up, vec![KeyCode::KEY_UP, KeyCode::KEY_K, KeyCode::KEY_2]);
    map.insert(Left, vec![KeyCode::KEY_LEFT, KeyCode::KEY_H, KeyCode::KEY_3]);
    map.insert(Down, vec![KeyCode::KEY_DOWN, KeyCode::KEY_J, KeyCode::KEY_4]);
    map.insert(Right, vec![KeyCode::KEY_RIGHT, KeyCode::KEY_L, KeyCode::KEY_1]);

    map.insert(R1, vec![KeyCode::BTN_LEFT]);
    map.insert(L1, vec![KeyCode::BTN_RIGHT]);
    map.insert(L2, vec![KeyCode::KEY_Q, KeyCode::BTN_EXTRA]);
    map.insert(R2, vec![KeyCode::KEY_X]);
    map.insert(L3, vec![KeyCode::KEY_LEFTALT]);
    map.insert(R3, vec![KeyCode::KEY_V]);

    map.insert(Select, vec![KeyCode::KEY_TAB]);
    map.insert(Start, vec![KeyCode::KEY_ENTER]);

    map.insert(Forward, vec![KeyCode::KEY_W]);
    map.insert(Port, vec![KeyCode::KEY_A]);
    map.insert(Backward, vec![KeyCode::KEY_S]);
    map.insert(Starboard, vec![KeyCode::KEY_D]);

    for button in ControllerButton::all() {
      assert!(map.contains_key(button), "Missing mapping for ControllerButton::{:?}", button);
    }

    map
  });

#[rustfmt::skip]
pub static KEYBOARD_BUTTON_MAP: LazyLock<HashMap<KeyCode, ControllerButton>> = LazyLock::new(|| {
  CONTROLLER_KEY_MAP
    .iter()
    .flat_map(|(button, keys)| keys.iter().map(move |key| (*key, *button)))
    .collect()
});
