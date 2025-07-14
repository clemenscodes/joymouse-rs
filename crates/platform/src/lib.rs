mod button;
mod event;
mod joystick;
mod keys;

use crate::{
  button::from_button_event_for_input_event, event::try_from_event_summary_for_controller_event,
};

use controller::{
  Axis, ButtonEvent, ControllerEvent, Direction, JoyStick, JoyStickEvent, JoyStickState, Polarity,
  State, Vector,
};
use settings::{MAX_STICK_TILT, MIN_STICK_TILT, SETTINGS};

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
              controller.lock().unwrap().handle_event(event);
            }
          }
        }
      }
    }
  }

  fn handle_event(&mut self, event: ControllerEvent) {
    match event {
      ControllerEvent::Button(event) => self.handle_button_event(event),
      ControllerEvent::JoyStick(event) => self.handle_joystick_event(event),
    }
  }

  pub fn handle_button_event(&mut self, event: ButtonEvent) {
    let button_event = from_button_event_for_input_event(event);
    self.emit_button_events(&[button_event]);
  }

  pub fn handle_joystick_event(&mut self, event: JoyStickEvent) {
    let joystick = event.joystick();
    let axis = event.axis();
    let polarity = event.polarity();
    let state = event.state();

    if *joystick == JoyStick::Left {
      self.update_left_stick_direction(axis, polarity, &state);
    }

    let direction = self.left_stick.lock().unwrap().direction();

    let vector = match joystick {
      JoyStick::Left => direction.map(Vector::from).unwrap_or_default().flipped_y(),
      JoyStick::Right => {
        let delta = f64::from(polarity);
        match axis {
          Axis::X => Vector::new(delta, 0.0),
          Axis::Y => Vector::new(0.0, delta),
        }
      }
    };

    if *joystick == JoyStick::Right {
      let vector = self.right_stick.lock().unwrap().micro(vector);
      self.move_right_stick(vector);
    } else {
      let vector = {
        let mut stick = self.left_stick.lock().unwrap();
        stick.tilt(vector)
      };
      self.move_left_stick(vector, None);
    }
  }

  fn update_left_stick_direction(&self, axis: &Axis, polarity: Polarity, state: &State) {
    let mut stick = self.left_stick.lock().unwrap();

    match axis {
      Axis::X => match polarity {
        Polarity::Negative(_) => stick.set_left(*state),
        Polarity::Positive(_) => stick.set_right(*state),
      },
      Axis::Y => match polarity {
        Polarity::Negative(_) => stick.set_down(*state),
        Polarity::Positive(_) => stick.set_up(*state),
      },
    }

    stick.update_direction();
  }

  fn virtual_device_mut(&mut self) -> &mut VirtualDevice {
    &mut self.virtual_device
  }

  fn emit_button_events(&mut self, events: &[InputEvent]) {
    self.virtual_device_mut().emit(events).unwrap()
  }

  fn emit_joystick_events(&mut self, events: &[InputEvent]) {
    self.virtual_device_mut().emit(events).unwrap()
  }

  fn left_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>> {
    &mut self.left_stick
  }

  fn right_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>> {
    &mut self.right_stick
  }

  fn monitor_left_stick(controller: Arc<Mutex<Self>>) {
    loop {
      {
        controller.lock().unwrap().handle_left_stick();
      }
      std::thread::sleep(std::time::Duration::from_millis(1));
    }
  }

  fn handle_left_stick(&mut self) {
    let maybe_direction = {
      let stick_lock = self.left_stick_mut();
      let stick = stick_lock.lock().unwrap();
      stick.direction()
    };

    if let Some(direction) = maybe_direction {
      let vector = Vector::from(direction) * settings::LEFT_STICK_SENSITIVITY;

      let vector = {
        let mut stick = self.left_stick_mut().lock().unwrap();
        stick.tilt(vector)
      };

      self.move_left_stick(vector, Some(direction));
    } else {
      self.center_left_stick();
    }
  }

  fn move_left_stick(&mut self, vector: Vector, direction: Option<Direction>) {
    let (x, y) = if let Some(direction) = direction {
      if direction == Direction::North {
        (0.0, -vector.dy() * 2.0)
      } else {
        (vector.dx(), -vector.dy())
      }
    } else {
      (vector.dx(), -vector.dy())
    };

    self.emit_button_events(&[
      Self::get_stick_event(JoyStick::Left, Axis::X, x),
      Self::get_stick_event(JoyStick::Left, Axis::Y, y),
    ]);
  }

  fn move_right_stick(&mut self, vector: Vector) {
    self.emit_joystick_events(&[
      Self::get_stick_event(JoyStick::Right, Axis::X, vector.dx()),
      Self::get_stick_event(JoyStick::Right, Axis::Y, vector.dy()),
    ]);
  }

  fn center_left_stick(&mut self) {
    self.move_left_stick(Vector::default(), None);
  }

  fn center_right_stick(&mut self) {
    self.move_right_stick(Vector::default());
  }

  fn monitor_right_stick(controller: Arc<Mutex<Self>>) {
    loop {
      {
        controller.lock().unwrap().handle_right_stick();
      }
      std::thread::sleep(SETTINGS.tickrate());
    }
  }

  fn handle_right_stick(&mut self) {
    let left_stick_direction = { self.left_stick.lock().unwrap().direction() };
    if self.right_stick_mut().lock().unwrap().handle_idle(left_stick_direction) {
      self.center_right_stick();
    }
  }

  fn get_stick_event(stick: JoyStick, axis: Axis, value: f64) -> InputEvent {
    let code = match (stick, axis) {
      (JoyStick::Left, Axis::X) => AbsoluteAxisCode::ABS_X,
      (JoyStick::Left, Axis::Y) => AbsoluteAxisCode::ABS_Y,
      (JoyStick::Right, Axis::X) => AbsoluteAxisCode::ABS_RX,
      (JoyStick::Right, Axis::Y) => AbsoluteAxisCode::ABS_RY,
    };

    InputEvent::new(EventType::ABSOLUTE.0, code.0, value as i32)
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
