use crate::ControllerButton;

use io::{AlphabeticKey, ArrowKey, Key, ModifierKey, MouseKey, NumericKey, SystemKey};

use std::{collections::HashMap, sync::LazyLock};

pub static JOYSTICK_KEYS: LazyLock<JoyStickKeys> = LazyLock::new(JoyStickKeys::default);

#[rustfmt::skip]
pub static CONTROLLER_KEY_MAP: LazyLock<HashMap<ControllerButton, Vec<Key>>> = LazyLock::new(|| {
  use ControllerButton::*;
  use Key::*;

  let mut map = HashMap::new();

  map.insert(South, vec![System(SystemKey::Space)]);
  map.insert(East, vec![Modifier(ModifierKey::LeftCtrl)]);
  map.insert(North, vec![Alphabetic(AlphabeticKey::F)]);
  map.insert(West, vec![Alphabetic(AlphabeticKey::C), Mouse(MouseKey::Side)]);

  map.insert(Up, vec![
    Arrow(ArrowKey::Up),
    Alphabetic(AlphabeticKey::K),
    Numeric(NumericKey::Num2),
  ]);
  map.insert(Left, vec![
    Arrow(ArrowKey::Left),
    Alphabetic(AlphabeticKey::H),
    Numeric(NumericKey::Num1),
  ]);
  map.insert(Down, vec![
    Arrow(ArrowKey::Down),
    Alphabetic(AlphabeticKey::J),
    Numeric(NumericKey::Num4),
  ]);
  map.insert(Right, vec![
    Arrow(ArrowKey::Right),
    Alphabetic(AlphabeticKey::L),
    Numeric(NumericKey::Num3),
  ]);

  map.insert(R1, vec![Mouse(MouseKey::Left)]);
  map.insert(L1, vec![Mouse(MouseKey::Right)]);
  map.insert(L2, vec![Alphabetic(AlphabeticKey::Q), Mouse(MouseKey::Extra)]);
  map.insert(R2, vec![Alphabetic(AlphabeticKey::X)]);
  map.insert(L3, vec![Modifier(ModifierKey::LeftAlt)]);
  map.insert(R3, vec![Alphabetic(AlphabeticKey::V)]);

  map.insert(Select, vec![System(SystemKey::Tab)]);
  map.insert(Start, vec![System(SystemKey::Enter)]);

  map.insert(Forward, vec![Alphabetic(AlphabeticKey::W)]);
  map.insert(Port, vec![Alphabetic(AlphabeticKey::A)]);
  map.insert(Backward, vec![Alphabetic(AlphabeticKey::S)]);
  map.insert(Starboard, vec![Alphabetic(AlphabeticKey::D)]);

  for button in ControllerButton::all() {
    assert!(
      map.contains_key(button),
      "Missing mapping for ControllerButton::{:?}",
      button
    );
  }

  map
});

#[rustfmt::skip]
pub static KEYBOARD_BUTTON_MAP: LazyLock<HashMap<Key, ControllerButton>> = LazyLock::new(|| {
  CONTROLLER_KEY_MAP
    .iter()
    .flat_map(|(button, keys)| keys.iter().map(move |key| (*key, *button)))
    .collect()
});

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
