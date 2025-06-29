mod controller;
mod mouse;

use crate::{
  controller::{Controller, ControllerEvent},
  mouse::Mouse,
};

use std::{
  os::fd::AsRawFd,
  sync::{Arc, Mutex},
};

use epoll::{ControlOptions::EPOLL_CTL_ADD, Event, Events};
use evdev::{Device, EventType, KeyCode, MiscCode, RelativeAxisCode};

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

fn process_input_events(mouse: Arc<Mutex<Device>>, keyboard: Arc<Mutex<Device>>, controller: Arc<Mutex<Controller>>) {
  let mouse_fd = mouse.lock().unwrap().as_raw_fd();
  let keyboard_fd = keyboard.lock().unwrap().as_raw_fd();

  let epoll_fd = epoll::create(false).expect("Failed to create epoll fd");

  let event_mouse = Event::new(Events::EPOLLIN, mouse_fd as u64);
  let event_keyboard = Event::new(Events::EPOLLIN, keyboard_fd as u64);

  epoll::ctl(epoll_fd, EPOLL_CTL_ADD, mouse_fd, event_mouse).unwrap();
  epoll::ctl(epoll_fd, EPOLL_CTL_ADD, keyboard_fd, event_keyboard).unwrap();

  let mut events = vec![Event::new(Events::empty(), 0); 2];

  loop {
    let num_events = epoll::wait(epoll_fd, -1, &mut events).unwrap();

    for epoll_event in events.iter().take(num_events) {
      let fd = epoll_event.data as i32;

      let device_opt = if fd == mouse_fd {
        Some(mouse.clone())
      } else if fd == keyboard_fd {
        Some(keyboard.clone())
      } else {
        None
      };

      if let Some(device) = device_opt {
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
