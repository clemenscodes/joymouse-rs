mod keys;

pub use keys::*;

use controller::ControllerButton;
use directories::BaseDirs;
use io::{AlphabeticKey, ArrowKey, Key, ModifierKey, MouseKey, NumericKey, SystemKey};

use std::{collections::HashMap, sync::LazyLock};

use serde::{Deserialize, Serialize};

#[rustfmt::skip]
pub static CONTROLLER_KEY_MAP: LazyLock<HashMap<ControllerButton, Vec<Key>>> = LazyLock::new(|| {
  let path = BaseDirs::new().unwrap().config_dir().join("joymouse/bindings.toml");

  if !path.exists() {
    let bindings = Bindings::default();
    let toml = toml::to_string(&bindings).expect("failed to serialize default bindings");
    std::fs::create_dir_all(path.parent().unwrap()).expect("failed to create config directory");
    std::fs::write(&path, toml).expect("failed to write default bindings file");
    return bindings.0;
  }

  match std::fs::read_to_string(&path).ok().and_then(|s| toml::from_str::<Bindings>(&s).ok()) {
    Some(bindings) => bindings.0,
    None => {
      eprintln!("Failed to load bindings from {:?}, falling back to defaults", path);
      Bindings::default().0
    }
  }
});

#[rustfmt::skip]
pub static KEYBOARD_BUTTON_MAP: LazyLock<HashMap<Key, ControllerButton>> = LazyLock::new(|| {
  CONTROLLER_KEY_MAP
    .iter()
    .flat_map(|(button, keys)| keys.iter().map(move |key| (*key, *button)))
    .collect()
});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Bindings(#[serde(with = "bindings_map_format")] pub HashMap<ControllerButton, Vec<Key>>);

impl Default for Bindings {
  fn default() -> Self {
    use ControllerButton::*;
    use Key::*;

    let mut map = HashMap::new();

    map.insert(South, vec![System(SystemKey::Space)]);
    map.insert(East, vec![Modifier(ModifierKey::LeftCtrl)]);
    map.insert(North, vec![Alphabetic(AlphabeticKey::F)]);
    map.insert(West, vec![Alphabetic(AlphabeticKey::C), Mouse(MouseKey::Side)]);

    map.insert(
      Up,
      vec![Arrow(ArrowKey::Up), Alphabetic(AlphabeticKey::K), Numeric(NumericKey::Num2)],
    );
    map.insert(
      Left,
      vec![Arrow(ArrowKey::Left), Alphabetic(AlphabeticKey::H), Numeric(NumericKey::Num1)],
    );
    map.insert(
      Down,
      vec![Arrow(ArrowKey::Down), Alphabetic(AlphabeticKey::J), Numeric(NumericKey::Num4)],
    );
    map.insert(
      Right,
      vec![Arrow(ArrowKey::Right), Alphabetic(AlphabeticKey::L), Numeric(NumericKey::Num3)],
    );

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
      assert!(map.contains_key(button), "Missing mapping for ControllerButton::{:?}", button);
    }

    Self(map)
  }
}

mod bindings_map_format {
  use controller::ControllerButton;
  use io::Key;
  use serde::de::Error;
  use serde::{Deserialize, Deserializer, Serializer};
  use std::collections::HashMap;

  pub fn serialize<S>(
    map: &HashMap<ControllerButton, Vec<Key>>,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    use controller::ControllerButton::*;
    use serde::ser::SerializeMap;

    let ordered_keys = [
      South, East, North, West, Up, Down, Left, Right, Forward, Backward, Starboard, Port, L1, R1,
      L2, R2, L3, R3, Start, Select,
    ];

    let mut ser = serializer.serialize_map(Some(map.len()))?;

    for button in ordered_keys {
      if let Some(keys) = map.get(&button) {
        let key_strs: Vec<String> = keys.iter().map(|k| k.to_string()).collect();
        ser.serialize_entry(&button.to_string(), &key_strs)?;
      }
    }

    ser.end()
  }

  pub fn deserialize<'de, D>(
    deserializer: D,
  ) -> Result<HashMap<ControllerButton, Vec<Key>>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let raw: HashMap<String, Vec<String>> = HashMap::deserialize(deserializer)?;
    let mut result = HashMap::new();

    for (button_str, key_strs) in raw {
      let button = button_str.parse::<ControllerButton>().map_err(D::Error::custom)?;
      let keys = key_strs
        .into_iter()
        .map(|s| Key::try_from(s.as_str()).map_err(D::Error::custom))
        .collect::<Result<Vec<_>, _>>()?;

      result.insert(button, keys);
    }

    Ok(result)
  }
}
