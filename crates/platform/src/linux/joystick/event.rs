use bindings::JOYSTICK_KEYS;
use controller::{JoyStick, JoyStickError, JoyStickEvent, Polarity, State};
use evdev::{KeyEvent, RelativeAxisEvent};

use crate::linux::{
  button::try_controller_button_from_keycode,
  joystick::{
    axis::{try_from_jk_kc_for_axis, try_from_relative_axis_code_for_axis},
    polarity::try_from_event_tuple_for_polarity,
    try_from_relative_axis_code_for_joystick,
  },
};

pub fn try_from_key_event_for_joystick_event(
  event: KeyEvent,
) -> Result<JoyStickEvent, JoyStickError> {
  let code = event.code();
  let joystick = JoyStick::Left;
  let axis = try_from_jk_kc_for_axis(&JOYSTICK_KEYS, code)?;
  let button = try_controller_button_from_keycode(code)?;
  let state = State::try_from(event.value())?;
  let polarity = try_from_event_tuple_for_polarity(&axis, &button, code)?;
  Ok(JoyStickEvent::new(joystick, axis, polarity, state))
}

pub fn try_from_relative_axis_event_for_joystick_event(
  event: RelativeAxisEvent,
) -> Result<JoyStickEvent, JoyStickError> {
  let (code, value) = event.destructure();
  let joystick = try_from_relative_axis_code_for_joystick(code)?;
  let axis = try_from_relative_axis_code_for_axis(code)?;
  let polarity = Polarity::from(value);
  let state = State::Pressed;
  Ok(JoyStickEvent::new(joystick, axis, polarity, state))
}
