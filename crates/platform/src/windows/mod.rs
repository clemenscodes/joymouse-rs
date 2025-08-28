use std::{
  sync::{Arc, Mutex},
  time::Duration,
};

use controller::{
  ControllerError, ControllerEvent, ControllerEventEmitter, JoyStickState,
  PlatformControllerManager, PlatformControllerOps, VirtualController, VirtualControllerCore,
};

use device_query::{DeviceEvents, DeviceEventsHandler, Keycode, MouseButton};
use vigem_client::{Client, TargetId, XButtons, XGamepad, Xbox360Wired};

pub struct Controller {
  virtual_device: Arc<Mutex<Xbox360Wired<Client>>>,
  left_stick: Arc<Mutex<JoyStickState>>,
  right_stick: Arc<Mutex<JoyStickState>>,
}

impl ControllerEventEmitter for Controller {
  fn emit(&mut self, _events: &[ControllerEvent]) -> Result<(), ControllerError> {
    let mut device = self.virtual_device.lock().unwrap();
    let gamepad = XGamepad {
      buttons: XButtons!(UP | RIGHT | LB | A | X),
      ..Default::default()
    };
    device.update(&gamepad).unwrap();
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
  type VirtualDevice = Xbox360Wired<Client>;
  type PhysicalDevice = ();

  fn init_mouse() -> Self::PhysicalDevice {
    ()
  }

  fn init_keyboard() -> Self::PhysicalDevice {
    ()
  }

  fn create_virtual_controller() -> Result<Self::VirtualDevice, Box<dyn std::error::Error>> {
    let client = Client::connect().unwrap();
    let target_id = TargetId::XBOX360_WIRED;
    let mut controller = vigem_client::Xbox360Wired::new(client, target_id);
    controller.plugin().unwrap();
    controller.wait_ready().unwrap();
    Ok(controller)
  }

  fn monitor_io(
    _mouse: Self::PhysicalDevice,
    _keyboard: Self::PhysicalDevice,
    _controller: Arc<Mutex<dyn VirtualControllerCore>>,
  ) -> ! {
    let handler = DeviceEventsHandler::new(Duration::from_millis(1))
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
      virtual_device: Arc::new(Mutex::new(WindowsOps::create_virtual_controller().unwrap())),
      left_stick: Arc::new(Mutex::new(JoyStickState::default())),
      right_stick: Arc::new(Mutex::new(JoyStickState::default())),
    })
  }
}
