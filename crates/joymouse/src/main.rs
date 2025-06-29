mod controller;
mod mouse;

use crate::{
  controller::{Controller, ControllerEvent, Vector},
  mouse::Mouse,
};

use std::{
  collections::HashMap,
  os::fd::{AsRawFd, RawFd},
  sync::{Arc, Mutex},
};

use epoll::{ControlOptions::EPOLL_CTL_ADD, Event, Events};
use evdev::{AbsoluteAxisCode, Device, EventType, InputEvent, KeyCode, MiscCode, RelativeAxisCode};

fn main() {
  let mut mice = find_mice();
  let mut keyboards = find_keyboards();

  let mouse = Arc::new(Mutex::new(mice.remove(0)));
  let xremap_keyboard =
    keyboards.iter().position(|keyboard| keyboard.name().is_some_and(|name| name.contains("xremap")));

  let keyboard = Arc::new(Mutex::new(if let Some(index) = xremap_keyboard {
    keyboards.remove(index)
  } else {
    keyboards.remove(0)
  }));

  let virtual_mouse = Mouse::try_create().unwrap();
  let controller = Arc::new(Mutex::new(Controller::try_create(virtual_mouse).unwrap()));

  {
    let controller = Arc::clone(&controller);
    std::thread::spawn(move || {
      let tick_rate = std::time::Duration::from_millis(16);
      const SPEED_PER_TICK: i32 = 10000;

      loop {
        std::thread::sleep(tick_rate);

        let mut controller = controller.lock().unwrap();

        let maybe_direction = {
          let stick = controller.left_stick_mut().lock().unwrap();
          stick.direction()
        };

        if let Some(direction) = maybe_direction {
          let vector = Vector::from(direction) * SPEED_PER_TICK;

          let (x, y) = {
            let mut stick = controller.left_stick_mut().lock().unwrap();
            stick.tilt(vector);
            (stick.x(), stick.y())
          };

          let events = vec![
            InputEvent::new(EventType::ABSOLUTE.0, AbsoluteAxisCode::ABS_X.0, x),
            InputEvent::new(EventType::ABSOLUTE.0, AbsoluteAxisCode::ABS_Y.0, -y),
          ];

          controller.virtual_device_mut().emit(&events).unwrap();
        }
      }
    });
  }

  {
    let controller = Arc::clone(&controller);
    std::thread::spawn(move || {
      let tick_rate = std::time::Duration::from_millis(16);
      let timeout = std::time::Duration::from_millis(100);

      loop {
        std::thread::sleep(tick_rate);

        let mut ctrl = controller.lock().unwrap();
        let now = std::time::Instant::now();

        let right_stick = ctrl.right_stick_mut().lock().unwrap();
        let last_mouse_event = right_stick.last_event();
        let elapsed = now.duration_since(last_mouse_event);
        let is_idle = elapsed > timeout;

        if is_idle && (right_stick.x() != 0 || right_stick.y() != 0) {
          drop(right_stick);
          ctrl.right_stick_mut().lock().unwrap().recenter();
          ctrl
            .virtual_device_mut()
            .emit(&[
              InputEvent::new(EventType::ABSOLUTE.0, AbsoluteAxisCode::ABS_RX.0, 0),
              InputEvent::new(EventType::ABSOLUTE.0, AbsoluteAxisCode::ABS_RY.0, 0),
            ])
            .unwrap();
        }
      }
    });
  }

  mouse.lock().unwrap().grab().unwrap();

  let mouse = Arc::clone(&mouse);
  let keyboard = Arc::clone(&keyboard);
  let controller = Arc::clone(&controller);

  let process = std::thread::spawn(move || {
    process_input_events(mouse, keyboard, controller);
  });

  println!("Started JoyMouse 🎮🐭");

  process.join().unwrap();
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

  candidates.sort_by_key(|device| extract_input_number(device.physical_path()).unwrap_or(u32::MAX));

  candidates
}

pub fn find_keyboards() -> Vec<Device> {
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

  candidates.sort_by_key(|device| extract_input_number(device.physical_path()).unwrap_or(u32::MAX));

  candidates
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

fn process_input_events(mouse: Arc<Mutex<Device>>, keyboard: Arc<Mutex<Device>>, controller: Arc<Mutex<Controller>>) {
  let epoll_fd = create_epoll_fd();

  let devices = [mouse, keyboard];
  let mut fd_map = HashMap::new();

  for device in devices.iter() {
    let fd = device.lock().unwrap().as_raw_fd();
    let event = Event::new(Events::EPOLLIN, fd as u64);
    epoll::ctl(epoll_fd, EPOLL_CTL_ADD, fd, event).unwrap();
    fd_map.insert(fd, Arc::clone(device));
  }

  let mut events = vec![Event::new(Events::empty(), 0); fd_map.len()];

  loop {
    let num_events = epoll::wait(epoll_fd, -1, &mut events).unwrap();

    for epoll_event in events.iter().take(num_events) {
      let fd = epoll_event.data as i32;

      if let Some(device) = fd_map.get(&fd) {
        let mut device = device.lock().unwrap();
        for original in device.fetch_events().unwrap() {
          if let Ok(event) = ControllerEvent::try_from(original.destructure()) {
            controller.lock().unwrap().handle_event(event, original);
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {}
