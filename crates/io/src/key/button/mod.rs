mod error;

pub use error::ButtonError;

use std::{collections::HashMap, sync::LazyLock};

use crate::{AlphabeticKey, ArrowKey, Key, ModifierKey, MouseKey, NumericKey, SystemKey};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerButton {
  South,
  East,
  North,
  West,
  Up,
  Down,
  Left,
  Right,
  Forward,
  Backward,
  Starboard,
  Port,
  L1,
  R1,
  L2,
  R2,
  L3,
  R3,
  Start,
  Select,
}

#[cfg(not(windows))]
impl TryFrom<ControllerButton> for evdev::KeyCode {
  type Error = ButtonError;

  fn try_from(value: ControllerButton) -> Result<Self, Self::Error> {
    let code = match value {
      ControllerButton::South => evdev::KeyCode::BTN_SOUTH,
      ControllerButton::East => evdev::KeyCode::BTN_EAST,
      ControllerButton::North => evdev::KeyCode::BTN_WEST,
      ControllerButton::West => evdev::KeyCode::BTN_NORTH,
      ControllerButton::Up => evdev::KeyCode::BTN_DPAD_UP,
      ControllerButton::Down => evdev::KeyCode::BTN_DPAD_DOWN,
      ControllerButton::Left => evdev::KeyCode::BTN_DPAD_LEFT,
      ControllerButton::Right => evdev::KeyCode::BTN_DPAD_RIGHT,
      ControllerButton::L1 => evdev::KeyCode::BTN_TL,
      ControllerButton::R1 => evdev::KeyCode::BTN_TR,
      ControllerButton::L2 => evdev::KeyCode::BTN_TL2,
      ControllerButton::R2 => evdev::KeyCode::BTN_TR2,
      ControllerButton::L3 => evdev::KeyCode::BTN_THUMBL,
      ControllerButton::R3 => evdev::KeyCode::BTN_THUMBR,
      ControllerButton::Start => evdev::KeyCode::BTN_START,
      ControllerButton::Select => evdev::KeyCode::BTN_SELECT,
      _ => return Err(ButtonError::InvalidButton(value)),
    };
    Ok(code)
  }
}

#[cfg(not(windows))]
impl TryFrom<ControllerButton> for Key {
  type Error = ButtonError;

  fn try_from(value: ControllerButton) -> Result<Self, Self::Error> {
    let code = evdev::KeyCode::try_from(value)?;
    let key = Key::try_from(code)?;
    Ok(key)
  }
}

impl TryFrom<Key> for ControllerButton {
  type Error = ButtonError;

  fn try_from(value: Key) -> Result<Self, Self::Error> {
    if let Some(button) = KEYBOARD_BUTTON_MAP.get(&value) {
      Ok(*button)
    } else {
      Err(ButtonError::InvalidKey(value.as_str().to_string()))
    }
  }
}

#[rustfmt::skip]
impl ControllerButton {
  pub fn all() -> &'static [Self] {
    use ControllerButton::*;
    &[
      South, East, North, West, 
      Up, Down, Left, Right, 
      Forward, Backward, Starboard, Port, 
      L1, R1, 
      L2, R2, 
      L3, R3,
      Start, Select,
    ]
  }
}

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
