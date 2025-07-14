use crate::joystick::{axis::JoyStickAxis, JoyStickError};

use controller::ControllerButton;
use io::Polarity;

use evdev::KeyCode;

pub fn try_from_event_tuple_for_polarity(
  axis: &JoyStickAxis,
  button: &ControllerButton,
  code: KeyCode,
) -> Result<Polarity, JoyStickError> {
  match axis {
    JoyStickAxis::X => match button {
      ControllerButton::Starboard => Ok(Polarity::Positive(1)),
      ControllerButton::Port => Ok(Polarity::Negative(-1)),
      _ => Err(JoyStickError::UnsupportedKeyCode(code)),
    },
    JoyStickAxis::Y => match button {
      ControllerButton::Forward => Ok(Polarity::Positive(1)),
      ControllerButton::Backward => Ok(Polarity::Negative(-1)),
      _ => Err(JoyStickError::UnsupportedKeyCode(code)),
    },
  }
}
