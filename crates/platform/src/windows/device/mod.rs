mod buttons;

use controller::{ControllerError, ControllerEvent, ControllerEventEmitter};
use vigem_client::{Client, TargetId, XGamepad, Xbox360Wired};

use crate::windows::device::buttons::CONTROLLER_BUTTONS;

#[derive(Debug)]
pub struct VirtualDevice {
  handle: Xbox360Wired<Client>,
}

impl VirtualDevice {
  pub fn handle_mut(&mut self) -> &mut Xbox360Wired<Client> {
    &mut self.handle
  }
}

impl Default for VirtualDevice {
  fn default() -> Self {
    let client = Client::connect().unwrap();
    let target_id = TargetId::XBOX360_WIRED;
    let mut handle = Xbox360Wired::new(client, target_id);
    handle.plugin().unwrap();
    handle.wait_ready().unwrap();
    Self {
      handle,
    }
  }
}

impl ControllerEventEmitter for VirtualDevice {
  fn emit(&mut self, _events: &[ControllerEvent]) -> Result<(), ControllerError> {
    let mut gamepad = XGamepad::default();
    gamepad.buttons = CONTROLLER_BUTTONS;
    self.handle_mut().update(&gamepad).unwrap();
    Ok(())
  }
}
