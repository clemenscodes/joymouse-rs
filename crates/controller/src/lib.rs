mod button;
mod error;
mod joystick;

use std::sync::{Arc, Mutex};

pub use button::*;
pub use error::*;
pub use joystick::*;

pub enum ControllerEvent {
  Button(ButtonEvent),
  JoyStick(JoyStickEvent),
}

pub trait ControllerEventEmitter: Send + Sync {
  fn emit(&mut self, events: &[ControllerEvent]) -> Result<(), ControllerError>;
}

pub struct VirtualController {
  emitter: Box<dyn ControllerEventEmitter>,
  left_stick: Arc<Mutex<JoyStickState>>,
  right_stick: Arc<Mutex<JoyStickState>>,
}

impl VirtualController {
  pub fn new(
    emitter: Box<dyn ControllerEventEmitter>,
    left_stick: Arc<Mutex<JoyStickState>>,
    right_stick: Arc<Mutex<JoyStickState>>,
  ) -> Self {
    Self {
      emitter,
      left_stick,
      right_stick,
    }
  }

  pub fn emit(&mut self, events: &[ControllerEvent]) -> Result<(), ControllerError> {
    self.emitter.emit(events)
  }

  pub fn left_stick(&self) -> &Mutex<JoyStickState> {
    &self.left_stick
  }

  pub fn right_stick(&self) -> &Mutex<JoyStickState> {
    &self.right_stick
  }

  pub fn left_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>> {
    &mut self.left_stick
  }

  pub fn right_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>> {
    &mut self.right_stick
  }
}
