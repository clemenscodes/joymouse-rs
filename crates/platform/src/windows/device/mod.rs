mod gamepad;

use controller::{ControllerError, ControllerEvent, JoyStickState};
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

    if let Err(err) = handle.plugin() {
      eprintln!("Failed to plugin virtual controller: {err}");
      std::process::exit(1);
    }

    std::thread::sleep(std::time::Duration::from_millis(2000));

    if let Err(err) = handle.wait_ready() {
      eprintln!("Failed to wait for virtual controller: {err} ");
      std::process::exit(1);
    }

    Self {
      handle,
      gamepad: Gamepad::default(),
    }
  }
}

impl VirtualDevice {
  pub fn emit(
    &mut self,
    events: &[ControllerEvent],
    left_stick: &JoyStickState,
    right_stick: &JoyStickState,
  ) -> Result<(), ControllerError> {
    for event in events {
      self.gamepad.update(event, left_stick, right_stick)?;

      if self.handle.update(&self.gamepad.handle()).is_err() {
        continue;
      }
    }

    Ok(())
  }

  pub fn disconnect(&mut self) -> Result<(), ControllerError> {
    if self.handle.unplug().is_err() {
      eprintln!("Failed to disconnect virtual controller");
      std::process::exit(1);
    }

    Ok(())
  }
}
