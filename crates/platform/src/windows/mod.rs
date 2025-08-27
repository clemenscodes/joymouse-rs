use std::{
  sync::{Arc, Mutex},
  time::Duration,
};

use controller::{
  ControllerError, ControllerEvent, ControllerEventEmitter, JoyStickState,
  PlatformControllerManager, PlatformControllerOps, VirtualController, VirtualControllerCore,
};

use device_query::{DeviceEvents, DeviceEventsHandler, Keycode, MouseButton};

pub struct Controller {
  left_stick: Arc<Mutex<JoyStickState>>,
  right_stick: Arc<Mutex<JoyStickState>>,
}

impl ControllerEventEmitter for Controller {
  fn emit(&mut self, _events: &[ControllerEvent]) -> Result<(), ControllerError> {
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

pub struct WindowsOps;

impl PlatformControllerOps for WindowsOps {
  type VirtualDevice = ();
  type PhysicalDevice = ();

  fn init_mouse() -> Self::PhysicalDevice {
    ()
  }

  fn init_keyboard() -> Self::PhysicalDevice {
    ()
  }

  fn create_virtual_controller() -> Result<Self::VirtualDevice, Box<dyn std::error::Error>> {
    todo!("[Windows] Integrate virtual gamepad using ViGEmBus driver and the vigem-client crate");
  }

  fn monitor_io(
    _mouse: Self::PhysicalDevice,
    _keyboard: Self::PhysicalDevice,
    _controller: Arc<Mutex<dyn VirtualControllerCore>>,
  ) -> ! {
    let handler = DeviceEventsHandler::new(Duration::from_millis(5))
      .expect("Failed to create DeviceEventsHandler");

    handler.on_key_down(|key: &Keycode| {
      println!("[key down ] {:?}", key);
    });
    handler.on_key_up(|key: &Keycode| {
      println!("[key up   ] {:?}", key);
    });
    handler.on_mouse_move(|pos: &(i32, i32)| {
      println!("[mouse move] x={}, y={}", pos.0, pos.1);
    });
    handler.on_mouse_down(|btn: &MouseButton| {
      println!("[mouse down] {:?}", btn);
    });
    handler.on_mouse_up(|btn: &MouseButton| {
      println!("[mouse up  ] {:?}", btn);
    });

    loop {
      std::thread::sleep(Duration::from_secs(1));
    }
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
