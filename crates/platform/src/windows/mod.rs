use std::sync::{Arc, Mutex};

use controller::{
  ControllerError, ControllerEvent, ControllerEventEmitter, JoyStickState, VirtualController,
};

pub struct Controller {
  left_stick: Arc<Mutex<JoyStickState>>,
  right_stick: Arc<Mutex<JoyStickState>>,
}

impl ControllerEventEmitter for Controller {
  fn emit(&mut self, events: &[ControllerEvent]) -> Result<(), ControllerError> {
    println!("Emitting events: {:#?}", events);
    Ok(())
  }
}

impl VirtualController for Controller {
  fn left_stick(&self) -> &Mutex<JoyStickState> {
    &self.left_stick
  }

  fn left_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>> {
    &mut self.left_stick
  }

  fn right_stick(&self) -> &Mutex<JoyStickState> {
    &self.right_stick
  }

  fn right_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>> {
    &mut self.right_stick
  }
}

impl Controller {
  pub fn run() {
    println!("Windows is not supported yet. Stay tuned!");
  }
}
