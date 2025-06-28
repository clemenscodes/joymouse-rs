use evdev::{KeyCode, KeyEvent, RelativeAxisCode, RelativeAxisEvent};

#[derive(Debug)]
pub struct AxisError;

#[derive(Debug)]
pub enum JoyStickAxis {
  X,
  Y,
}

impl TryFrom<RelativeAxisCode> for JoyStickAxis {
  type Error = AxisError;

  fn try_from(value: RelativeAxisCode) -> Result<Self, Self::Error> {
    let axis = match value {
      RelativeAxisCode::REL_X => Self::X,
      RelativeAxisCode::REL_Y => Self::Y,
      _ => return Err(AxisError),
    };
    Ok(axis)
  }
}

impl TryFrom<RelativeAxisEvent> for JoyStickAxis {
  type Error = AxisError;

  fn try_from(value: RelativeAxisEvent) -> Result<Self, Self::Error> {
    Self::try_from(value.code())
  }
}
