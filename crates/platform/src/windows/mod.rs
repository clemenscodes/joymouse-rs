use std::sync::{Arc, Mutex};

use controller::{
  ControllerError, ControllerEvent, ControllerEventEmitter, JoyStickState,
  PlatformControllerManager, PlatformControllerOps, VirtualController, VirtualControllerCore,
};

pub struct Controller {
  left_stick: Arc<Mutex<JoyStickState>>,
  right_stick: Arc<Mutex<JoyStickState>>,
}

impl ControllerEventEmitter for Controller {
  fn emit(&mut self, events: &[ControllerEvent]) -> Result<(), ControllerError> {
    todo!("[Windows] Emit controller events: {:#?}", events);
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

pub struct WindowsOps;

impl PlatformControllerOps for WindowsOps {
  type VirtualDevice = ();
  type PhysicalDevice = ();

  fn create_virtual_controller() -> Result<Self::VirtualDevice, Box<dyn std::error::Error>> {
    todo!("[Windows] Integrate virtual gamepad using ViGEmBus driver and the vigem-client crate");
  }

  fn init_mouse() -> Self::PhysicalDevice {
    todo!("[Windows] Use device_query to detect mouse");
  }

  fn init_keyboard() -> Self::PhysicalDevice {
    todo!("[Windows] Use device_query to detect keyboard");
  }

  fn monitor_io(
    _mouse: Self::PhysicalDevice,
    _keyboard: Self::PhysicalDevice,
    _controller: Arc<Mutex<dyn VirtualControllerCore>>,
  ) -> ! {
    todo!("[Windows] Implement event loop for Raw Input (mouse + keyboard)");
  }
}

impl PlatformControllerManager for Controller {
  type Ops = WindowsOps;

  fn try_create() -> Result<Self, Box<dyn std::error::Error>> {
    Ok(Self {
      left_stick: Arc::new(Mutex::new(JoyStickState::default())),
      right_stick: Arc::new(Mutex::new(JoyStickState::default())),
    })
  }
}
