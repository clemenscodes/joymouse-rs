mod axis;
mod event;
mod keys;
mod polarity;

pub use event::*;

use controller::{JoyStick, JoyStickError};

use evdev::RelativeAxisCode;

pub fn try_from_relative_axis_code_for_joystick(
  code: RelativeAxisCode,
) -> Result<JoyStick, JoyStickError> {
  let joystick = match code {
    RelativeAxisCode::REL_X => JoyStick::Right,
    RelativeAxisCode::REL_Y => JoyStick::Right,
    other => return Err(JoyStickError::UnsupportedCode(other.0)),
  };
  Ok(joystick)
}

pub use keys::JOYSTICK_KEYS;
