mod button;
mod event;
mod joystick;

use crate::linux::event::{
  from_controller_event_for_input_event, try_from_event_summary_for_controller_event,
};

use controller::{
  ControllerError, ControllerEvent, ControllerEventEmitter, JoyStickState,
  PlatformControllerManager, PlatformControllerOps, VirtualController, VirtualControllerCore,
};
use settings::{MAX_STICK_TILT, MIN_STICK_TILT};

use std::{
  collections::HashMap,
  os::fd::{AsRawFd, RawFd},
  sync::{Arc, Mutex},
};

use epoll::{Event, Events};
use evdev::{
  uinput::VirtualDevice, AbsInfo, AbsoluteAxisCode, AttributeSet, BusType, Device, EventType,
  InputEvent, InputId, KeyCode, MiscCode, RelativeAxisCode, UinputAbsSetup,
};

#[derive(Debug)]
pub struct Controller {
  virtual_device: VirtualDevice,
  left_stick: Arc<Mutex<JoyStickState>>,
  right_stick: Arc<Mutex<JoyStickState>>,
}

#[rustfmt::skip]
impl ControllerEventEmitter for Controller {
    fn emit(&mut self, events: &[ControllerEvent]) -> Result<(), ControllerError> {
        let input_events: Vec<InputEvent> = events
            .iter()
            .map(|e| from_controller_event_for_input_event(*e))
            .collect();
        self.virtual_device.emit(&input_events).unwrap();
        Ok(())
    }
}

impl VirtualController for Controller {
  fn left_stick(&self) -> &Mutex<JoyStickState> {
    &self.left_stick
  }

  fn right_stick(&self) -> &Mutex<JoyStickState> {
    &self.right_stick
  }

  fn left_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>> {
    &mut self.left_stick
  }

  fn right_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>> {
    &mut self.right_stick
  }
}

impl PlatformControllerManager for Controller {
  type Ops = LinuxOps;

  fn try_create() -> Result<Self, Box<dyn std::error::Error>> {
    Ok(Self {
      virtual_device: LinuxOps::create_virtual_controller()?,
      left_stick: Arc::new(Mutex::new(JoyStickState::default())),
      right_stick: Arc::new(Mutex::new(JoyStickState::default())),
    })
  }
}

pub struct LinuxOps;

impl PlatformControllerOps for LinuxOps {
  type VirtualDevice = VirtualDevice;
  type PhysicalDevice = Device;

  fn create_virtual_controller() -> Result<VirtualDevice, Box<dyn std::error::Error>> {
    let builder = VirtualDevice::builder()?;

    let name = "JoyMouse";
    let bus_type = BusType::BUS_USB;
    let vendor = 0x1234;
    let product = 0x5678;
    let version = 0x0100;
    let deadzone = 0;
    let noise_tolerance = 0;
    let min = MIN_STICK_TILT as i32;
    let max = MAX_STICK_TILT as i32;
    let input_id = InputId::new(bus_type, vendor, product, version);

    let mut button_set = AttributeSet::<KeyCode>::new();

    let buttons = [
      KeyCode::BTN_SOUTH,
      KeyCode::BTN_SOUTH,
      KeyCode::BTN_EAST,
      KeyCode::BTN_NORTH,
      KeyCode::BTN_WEST,
      KeyCode::BTN_TL,
      KeyCode::BTN_TR,
      KeyCode::BTN_TL2,
      KeyCode::BTN_TR2,
      KeyCode::BTN_START,
      KeyCode::BTN_SELECT,
      KeyCode::BTN_THUMBL,
      KeyCode::BTN_THUMBR,
      KeyCode::BTN_DPAD_UP,
      KeyCode::BTN_DPAD_DOWN,
      KeyCode::BTN_DPAD_LEFT,
      KeyCode::BTN_DPAD_RIGHT,
    ];

    for button in buttons {
      button_set.insert(button);
    }

    let axis_info = AbsInfo::new(0, min, max, noise_tolerance, deadzone, 0);
    let x_axis = UinputAbsSetup::new(AbsoluteAxisCode::ABS_X, axis_info);
    let y_axis = UinputAbsSetup::new(AbsoluteAxisCode::ABS_Y, axis_info);
    let rx_axis = UinputAbsSetup::new(AbsoluteAxisCode::ABS_RX, axis_info);
    let ry_axis = UinputAbsSetup::new(AbsoluteAxisCode::ABS_RY, axis_info);

    let virtual_device = builder
      .name(name)
      .input_id(input_id)
      .with_keys(&button_set)?
      .with_absolute_axis(&x_axis)?
      .with_absolute_axis(&y_axis)?
      .with_absolute_axis(&rx_axis)?
      .with_absolute_axis(&ry_axis)?
      .build()?;

    Ok(virtual_device)
  }

  fn init_mouse() -> Self::PhysicalDevice {
    let mut mice = Self::find_mice();
    Self::find_mouse(&mut mice)
  }

  fn init_keyboard() -> Self::PhysicalDevice {
    let mut candidates = Self::find_keyboards();
    Self::find_keyboard(&mut candidates)
  }

  fn monitor_io(
    mouse: Self::PhysicalDevice,
    keyboard: Self::PhysicalDevice,
    controller: Arc<Mutex<dyn VirtualControllerCore>>,
  ) -> ! {
    let epoll_fd = Self::create_epoll_fd();

    let mut fd_map: HashMap<i32, Self::PhysicalDevice> = HashMap::new();

    {
      let fd = mouse.as_raw_fd();
      let event = Event::new(Events::EPOLLIN, fd as u64);
      epoll::ctl(epoll_fd, epoll::ControlOptions::EPOLL_CTL_ADD, fd, event).unwrap();
      fd_map.insert(fd, mouse);
    }

    {
      let fd = keyboard.as_raw_fd();
      let event = Event::new(Events::EPOLLIN, fd as u64);
      epoll::ctl(epoll_fd, epoll::ControlOptions::EPOLL_CTL_ADD, fd, event).unwrap();
      fd_map.insert(fd, keyboard);
    }

    let mut events = vec![Event::new(Events::empty(), 0); fd_map.len()];

    loop {
      let num_events = epoll::wait(epoll_fd, -1, &mut events).unwrap();

      for epoll_event in events.iter().take(num_events) {
        let fd = epoll_event.data as i32;

        if let Some(device) = fd_map.get_mut(&fd) {
          for event in device.fetch_events().unwrap() {
            if let Ok(event) = try_from_event_summary_for_controller_event(event.destructure()) {
              controller.lock().unwrap().handle_event(event).unwrap();
            }
          }
        }
      }
    }
  }
}

impl LinuxOps {
  fn extract_input_number(phys: Option<&str>) -> Option<u32> {
    phys
      .and_then(|s| s.split('/').next_back())
      .and_then(|last| last.strip_prefix("input"))
      .and_then(|n| n.parse::<u32>().ok())
  }

  fn create_epoll_fd() -> RawFd {
    match epoll::create(false) {
      Ok(fd) => fd,
      Err(_) => {
        eprintln!("Failed to create epoll file descriptor");
        std::process::exit(1);
      }
    }
  }

  fn find_mice() -> Vec<Device> {
    let mut candidates: Vec<Device> = evdev::enumerate()
      .filter(|(_, device)| {
        let events = device.supported_events();
        if !events.contains(EventType::RELATIVE) {
          return false;
        }

        let relative_axes = device.supported_relative_axes().unwrap_or_default();
        if !relative_axes.contains(RelativeAxisCode::REL_X) {
          return false;
        }

        let keys = device.supported_keys().unwrap_or_default();
        if !keys.contains(KeyCode::BTN_LEFT) {
          return false;
        }

        let misc = device.misc_properties().unwrap_or_default();
        if !misc.contains(MiscCode::MSC_SCAN) {
          return false;
        }

        let name = device.name().unwrap_or("");
        if name.contains("Receiver") {
          return false;
        }

        true
      })
      .map(|(_, device)| device)
      .collect();

    candidates
      .sort_by_key(|device| Self::extract_input_number(device.physical_path()).unwrap_or(u32::MAX));

    candidates
  }

  fn find_mouse(candidates: &mut Vec<Device>) -> Device {
    candidates.remove(0)
  }

  fn find_keyboards() -> Vec<Device> {
    let mut candidates: Vec<Device> = evdev::enumerate()
      .filter(|(_, device)| {
        let events = device.supported_events();
        if !events.contains(EventType::KEY) {
          return false;
        }

        let keys = device.supported_keys().unwrap_or_default();
        if !keys.contains(KeyCode::KEY_A) {
          return false;
        }

        let name = device.name().unwrap_or("");
        if name.contains("Receiver")
          || name.contains("Mouse")
          || name.contains("Yubico")
          || name.contains("ydotool")
          || name.contains("virtual")
        {
          return false;
        }

        true
      })
      .map(|(_, device)| device)
      .collect();

    candidates
      .sort_by_key(|device| Self::extract_input_number(device.physical_path()).unwrap_or(u32::MAX));

    candidates
  }

  fn find_keyboard(candidates: &mut Vec<Device>) -> Device {
    let index = candidates
      .iter()
      .position(|k| k.name().is_some_and(|name| name.contains("xremap")))
      .unwrap_or(0);
    candidates.remove(index)
  }
}
