use controller::{Axis, ControllerButton, JoyStickError, Polarity};

use evdev::KeyCode;

pub fn try_from_event_tuple_for_polarity(
  axis: &Axis,
  button: &ControllerButton,
  code: KeyCode,
) -> Result<Polarity, JoyStickError> {
  match axis {
    Axis::X => match button {
      ControllerButton::Starboard => Ok(Polarity::Positive(1)),
      ControllerButton::Port => Ok(Polarity::Negative(-1)),
      _ => Err(JoyStickError::UnsupportedCode(code.code())),
    },
    Axis::Y => match button {
      ControllerButton::Forward => Ok(Polarity::Positive(1)),
      ControllerButton::Backward => Ok(Polarity::Negative(-1)),
      _ => Err(JoyStickError::UnsupportedCode(code.code())),
    },
  }
}
