use bindings::JoyStickKeys;
use controller::{Axis, AxisError};

use evdev::{KeyCode, RelativeAxisCode};
use io::Key;

pub fn try_from_jk_kc_for_axis(keys: &JoyStickKeys, code: KeyCode) -> Result<Axis, AxisError> {
  let key = Key::try_from(code).map_err(|_| AxisError::Unknown)?;
  if !keys.key_is_joystick_key(key) {
    return Err(AxisError::Unknown);
  }

  let axis = if keys.forward().contains(&key) || keys.backward().contains(&key) {
    Axis::Y
  } else {
    Axis::X
  };

  Ok(axis)
}

pub fn try_from_relative_axis_code_for_axis(code: RelativeAxisCode) -> Result<Axis, AxisError> {
  let axis = match code {
    RelativeAxisCode::REL_X => Axis::X,
    RelativeAxisCode::REL_Y => Axis::Y,
    _ => return Err(AxisError::Unknown),
  };
  Ok(axis)
}


