use controller::{Axis, JoyStick, JoyStickError, Polarity, State};
use evdev::{AbsoluteAxisCode, KeyEvent, RelativeAxisEvent};

use crate::{
  button::try_controller_button_from_keycode,
  joystick::{
    axis::{try_from_jk_kc_for_axis, try_from_relative_axis_code_for_axis},
    polarity::try_from_event_tuple_for_polarity,
    try_from_relative_axis_code_for_joystick, JOYSTICK_KEYS,
  },
};

#[derive(Debug)]
pub struct ControllerJoyStickEvent {
  joystick: JoyStick,
  axis: Axis,
  polarity: Polarity,
  state: State,
}

impl ControllerJoyStickEvent {
  pub fn new(joystick: JoyStick, axis: Axis, polarity: Polarity, state: State) -> Self {
    Self {
      joystick,
      axis,
      polarity,
      state,
    }
  }

  pub fn joystick(&self) -> &JoyStick {
    &self.joystick
  }

  pub fn axis(&self) -> &Axis {
    &self.axis
  }

  pub fn polarity(&self) -> Polarity {
    self.polarity
  }

  pub fn state(&self) -> &State {
    &self.state
  }
}

impl TryFrom<KeyEvent> for ControllerJoyStickEvent {
  type Error = JoyStickError;

  fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
    let code = value.code();
    let joystick = JoyStick::Left;
    let axis = try_from_jk_kc_for_axis(&JOYSTICK_KEYS, code)?;
    let button = try_controller_button_from_keycode(code)?;
    let state = State::try_from(value.value())?;
    let polarity = try_from_event_tuple_for_polarity(&axis, &button, code)?;
    Ok(Self::new(joystick, axis, polarity, state))
  }
}

impl TryFrom<RelativeAxisEvent> for ControllerJoyStickEvent {
  type Error = JoyStickError;

  fn try_from(value: RelativeAxisEvent) -> Result<Self, Self::Error> {
    let (code, value) = value.destructure();
    let joystick = try_from_relative_axis_code_for_joystick(code)?;
    let axis = try_from_relative_axis_code_for_axis(code)?;
    let polarity = Polarity::try_from(value as f64)?;
    let state = State::Pressed;
    Ok(Self::new(joystick, axis, polarity, state))
  }
}

impl From<&ControllerJoyStickEvent> for AbsoluteAxisCode {
  fn from(value: &ControllerJoyStickEvent) -> Self {
    match value.joystick() {
      JoyStick::Left => match value.axis() {
        Axis::X => Self::ABS_X,
        Axis::Y => Self::ABS_Y,
      },
      JoyStick::Right => match value.axis() {
        Axis::X => Self::ABS_RX,
        Axis::Y => Self::ABS_RY,
      },
    }
  }
}
