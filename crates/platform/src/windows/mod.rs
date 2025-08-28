mod device;

use crate::windows::device::VirtualDevice;

use controller::{
  ControllerError, ControllerEvent, ControllerEventEmitter, JoyStickState,
  PlatformControllerManager, PlatformControllerOps, VirtualController, VirtualControllerCore,
};

use std::{
  sync::{Arc, Mutex},
  time::Duration,
};

use device_query::{DeviceEvents, DeviceEventsHandler, Keycode, MouseButton};

pub struct Controller {
  virtual_device: Arc<Mutex<VirtualDevice>>,
  left_stick: Arc<Mutex<JoyStickState>>,
  right_stick: Arc<Mutex<JoyStickState>>,
}

impl ControllerEventEmitter for Controller {
  fn emit(&mut self, events: &[ControllerEvent]) -> Result<(), ControllerError> {
    self.virtual_device.lock().unwrap().emit(events)
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
  type VirtualDevice = VirtualDevice;
  type PhysicalDevice = ();

  fn init_mouse() -> Self::PhysicalDevice {
    ()
  }

  fn init_keyboard() -> Self::PhysicalDevice {
    ()
  }

  fn create_virtual_controller() -> Result<Self::VirtualDevice, Box<dyn std::error::Error>> {
    let controller = Self::VirtualDevice::default();
    Ok(controller)
  }

  fn monitor_io(
    _mouse: Self::PhysicalDevice,
    _keyboard: Self::PhysicalDevice,
    _controller: Arc<Mutex<dyn VirtualControllerCore>>,
  ) -> ! {
    let handler = DeviceEventsHandler::new(Duration::from_millis(1))
      .expect("Failed to create DeviceEventsHandler");

    let _g_key_down = handler.on_key_down(|key: &Keycode| {
      println!("[key down ] {:?}", key);
    });
    let _g_key_up = handler.on_key_up(|key: &Keycode| {
      println!("[key up   ] {:?}", key);
    });
    let _g_mouse_move = handler.on_mouse_move(|pos: &(i32, i32)| {
      println!("[mouse move] x={}, y={}", pos.0, pos.1);
    });
    let _g_mouse_down = handler.on_mouse_down(|btn: &MouseButton| {
      println!("[mouse down] {:?}", btn);
    });
    let _g_mouse_up = handler.on_mouse_up(|btn: &MouseButton| {
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
      virtual_device: Arc::new(Mutex::new(WindowsOps::create_virtual_controller().unwrap())),
      left_stick: Arc::new(Mutex::new(JoyStickState::default())),
      right_stick: Arc::new(Mutex::new(JoyStickState::default())),
    })
  }
}
