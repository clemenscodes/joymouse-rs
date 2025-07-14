use crate::joystick::keys::JoyStickKeys;

use controller::{Axis, AxisError};

use evdev::{KeyCode, RelativeAxisCode};

pub fn try_from_jk_kc_for_axis(keys: &JoyStickKeys, code: KeyCode) -> Result<Axis, AxisError> {
  if !keys.code_is_joystick_key(code) {
    return Err(AxisError::Unknown);
  }

  let axis = if keys.forward().contains(&code) || keys.backward().contains(&code) {
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
