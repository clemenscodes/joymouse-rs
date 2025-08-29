mod buttons;
mod gamepad;

use controller::{ControllerError, ControllerEvent, ControllerEventEmitter};
use vigem_client::{Client, TargetId, XTarget};

use crate::windows::device::gamepad::Gamepad;

#[derive(Debug)]
pub struct VirtualDevice {
  handle: XTarget,
  gamepad: Gamepad,
}

impl Default for VirtualDevice {
  fn default() -> Self {
    let client = Client::connect().unwrap();
    let target_id = TargetId::XBOX360_WIRED;
    let mut handle = XTarget::new(client, target_id);
    handle.plugin().unwrap();
    Self {
      handle,
      gamepad: Gamepad::default(),
    }
  }
}

impl ControllerEventEmitter for VirtualDevice {
  fn emit(&mut self, events: &[ControllerEvent]) -> Result<(), ControllerError> {
    for event in events {
      self.gamepad.update(event)?;

      if self.handle.wait_ready().is_ok() {
        if self.handle.update(&self.gamepad.handle()).is_err() {
          continue;
        }
      }
    }

    Ok(())
  }

  fn disconnect(&mut self) -> Result<(), ControllerError> {
    self.handle.unplug().unwrap();
    Ok(())
  }
}
