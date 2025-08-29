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
    let client = match Client::connect() {
      Ok(client) => client,
      Err(_) => {
        eprintln!("Failed to connect to ViGEmBus driver. Did you install it?");
        std::process::exit(1);
      }
    };

    let target_id = TargetId::XBOX360_WIRED;
    let mut handle = XTarget::new(client, target_id);

    if handle.plugin().is_err() {
      eprintln!("Failed to plugin virtual controller");
      std::process::exit(1);
    }

    if handle.wait_ready().is_err() {
      eprintln!("Failed to wait for virtual controller");
      std::process::exit(1);
    }

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

      if self.handle.update(&self.gamepad.handle()).is_err() {
        continue;
      }
    }

    Ok(())
  }

  fn disconnect(&mut self) -> Result<(), ControllerError> {
    if self.handle.unplug().is_err() {
      eprintln!("Failed to disconnect virtual controller");
      std::process::exit(1);
    }

    Ok(())
  }
}
