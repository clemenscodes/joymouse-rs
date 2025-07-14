mod button;
mod event;
mod joystick;

use crate::linux::event::try_from_event_summary_for_controller_event;

use controller::{
  ButtonEvent, ControllerButton, ControllerError, ControllerEvent, ControllerEventEmitter,
  JoyStickEvent, JoyStickState, VirtualController,
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

pub fn from_controller_event_for_input_event(event: ControllerEvent) -> InputEvent {
  match event {
    ControllerEvent::Button(button_event) => from_button_event_for_input_event(button_event),
    ControllerEvent::JoyStick(joystick_event) => {
      from_joystick_event_for_input_event(joystick_event)
    }
  }
}

pub fn from_button_event_for_input_event(event: ButtonEvent) -> InputEvent {
  let value: i32 = (*event.state()).into();
  let code = match event.button() {
    ControllerButton::South => KeyCode::BTN_SOUTH,
    ControllerButton::East => todo!(),
    ControllerButton::North => todo!(),
    ControllerButton::West => todo!(),
    ControllerButton::Up => todo!(),
    ControllerButton::Down => todo!(),
    ControllerButton::Left => todo!(),
    ControllerButton::Right => todo!(),
    ControllerButton::L1 => todo!(),
    ControllerButton::R1 => todo!(),
    ControllerButton::L2 => todo!(),
    ControllerButton::R2 => todo!(),
    ControllerButton::L3 => todo!(),
    ControllerButton::R3 => todo!(),
    ControllerButton::Start => todo!(),
    ControllerButton::Select => todo!(),
    _ => todo!(),
  };
  InputEvent::new(EventType::KEY.0, code.code(), value)
}

pub fn from_joystick_event_for_input_event(event: JoyStickEvent) -> InputEvent {
  todo!()
  // InputEvent::new(EventType::ABSOLUTE.0, code, value)
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

impl Controller {
  pub fn run() {
    let mouse = Arc::new(Mutex::new(Self::init_mouse()));
    let keyboard = Arc::new(Mutex::new(Self::init_keyboard()));
    let controller = Arc::new(Mutex::new(Self::try_create().unwrap()));

    let left_stick = Arc::clone(&controller);
    std::thread::spawn(move || Self::monitor_left_stick(left_stick));

    let right_stick = Arc::clone(&controller);
    std::thread::spawn(move || Self::monitor_right_stick(right_stick));

    let io = std::thread::spawn(move || Self::monitor_io(mouse, keyboard, controller));

    println!("Started JoyMouse 🎮🐭");

    io.join().unwrap();
  }

  fn try_create() -> Result<Self, Box<dyn std::error::Error>> {
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

    Ok(Self {
      virtual_device,
      left_stick: Arc::new(Mutex::new(JoyStickState::default())),
      right_stick: Arc::new(Mutex::new(JoyStickState::default())),
    })
  }

  fn init_mouse() -> Device {
    let mut mice = Self::find_mice();
    Self::find_mouse(&mut mice)
  }

  fn init_keyboard() -> Device {
    let mut candidates = Self::find_keyboards();
    Self::find_keyboard(&mut candidates)
  }

  fn monitor_io(
    mouse: Arc<Mutex<Device>>,
    keyboard: Arc<Mutex<Device>>,
    controller: Arc<Mutex<Self>>,
  ) {
    let epoll_fd = Self::create_epoll_fd();

    let devices = [mouse, keyboard];
    let mut fd_map = HashMap::new();

    for device in devices.iter() {
      let fd = device.lock().unwrap().as_raw_fd();
      let event = Event::new(Events::EPOLLIN, fd as u64);
      epoll::ctl(epoll_fd, epoll::ControlOptions::EPOLL_CTL_ADD, fd, event).unwrap();
      fd_map.insert(fd, Arc::clone(device));
    }

    let mut events = vec![Event::new(Events::empty(), 0); fd_map.len()];

    loop {
      let num_events = epoll::wait(epoll_fd, -1, &mut events).unwrap();

      for epoll_event in events.iter().take(num_events) {
        let fd = epoll_event.data as i32;

        if let Some(device) = fd_map.get(&fd) {
          let mut device = device.lock().unwrap();
          for event in device.fetch_events().unwrap() {
            if let Ok(event) = try_from_event_summary_for_controller_event(event.destructure()) {
              controller.lock().unwrap().handle_event(event).unwrap();
            }
          }
        }
      }
    }
  }

  fn extract_input_number(phys: Option<&str>) -> Option<u32> {
    phys
      .as_ref()
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

        let relative_axes = match device.supported_relative_axes() {
          Some(axes) => axes,
          None => return false,
        };

        if !relative_axes.contains(RelativeAxisCode::REL_X) {
          return false;
        }

        let keys = match device.supported_keys() {
          Some(keys) => keys,
          None => return false,
        };

        if !keys.contains(KeyCode::BTN_LEFT) {
          return false;
        }

        let misc = match device.misc_properties() {
          Some(misc) => misc,
          None => return false,
        };

        if !misc.contains(MiscCode::MSC_SCAN) {
          return false;
        }

        let name = match device.name() {
          Some(name) => name,
          None => return false,
        };

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

        let keys = match device.supported_keys() {
          Some(keys) => keys,
          None => return false,
        };

        if !keys.contains(KeyCode::KEY_A) {
          return false;
        }

        let name = match device.name() {
          Some(name) => name,
          None => return false,
        };

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
