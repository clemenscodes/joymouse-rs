use crate::{ButtonEvent, JoyStickEvent};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ControllerEvent {
  Button(ButtonEvent),
  JoyStick(JoyStickEvent),
}

impl From<ButtonEvent> for ControllerEvent {
  fn from(v: ButtonEvent) -> Self {
    Self::Button(v)
  }
}

impl From<JoyStickEvent> for ControllerEvent {
  fn from(v: JoyStickEvent) -> Self {
    Self::JoyStick(v)
  }
}
