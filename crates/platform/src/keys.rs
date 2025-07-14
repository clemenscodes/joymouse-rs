use controller::ControllerButton;
use evdev::KeyCode;
use std::{collections::HashMap, sync::LazyLock};

#[rustfmt::skip]
pub static CONTROLLER_KEY_MAP: LazyLock<HashMap<ControllerButton, Vec<KeyCode>>> = LazyLock::new(|| {
    use ControllerButton::*;

    let mut map = HashMap::new();

    map.insert(South, vec![KeyCode::KEY_SPACE]);
    map.insert(East, vec![KeyCode::KEY_LEFTCTRL]);
    map.insert(North, vec![KeyCode::KEY_F]);
    map.insert(West, vec![KeyCode::KEY_C, KeyCode::BTN_SIDE]);

    map.insert(Up, vec![KeyCode::KEY_UP, KeyCode::KEY_K, KeyCode::KEY_2]);
    map.insert(Left, vec![KeyCode::KEY_LEFT, KeyCode::KEY_H, KeyCode::KEY_1]);
    map.insert(Down, vec![KeyCode::KEY_DOWN, KeyCode::KEY_J, KeyCode::KEY_4]);
    map.insert(Right, vec![KeyCode::KEY_RIGHT, KeyCode::KEY_L, KeyCode::KEY_3]);

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
