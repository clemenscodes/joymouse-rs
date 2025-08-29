use controller::{ControllerError, ControllerEvent};
use vigem_client::XGamepad;

use crate::windows::device::buttons::CONTROLLER_BUTTONS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gamepad {
  handle: XGamepad,
}

impl Default for Gamepad {
  fn default() -> Self {
    let mut gamepad = XGamepad::default();
    gamepad.buttons = CONTROLLER_BUTTONS;
    Self {
      handle: gamepad,
    }
  }
}

impl Gamepad {
  pub fn handle(&self) -> XGamepad {
    self.handle
  }

  pub fn update(&mut self, event: &ControllerEvent) -> Result<(), ControllerError> {
    println!("[Updating gamepad from IO event]: {:#?}", event);
    Ok(())
  }
}
