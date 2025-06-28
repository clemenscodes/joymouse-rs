mod controller;

use crate::controller::{Controller, ControllerError, ControllerEvent};

use std::{
  os::fd::AsRawFd,
  sync::{Arc, Mutex},
};

use epoll::{ControlOptions::EPOLL_CTL_ADD, Event, Events};
use evdev::{Device, EventType, KeyCode, MiscCode, RelativeAxisCode};

fn main() {
  let mut mice = find_mice();

  let mut keyboards = find_keyboards();

  let mouse = mice.remove(0);

  let xremap_keyboard = keyboards.iter().position(|k| k.name().map(|n| n.contains("xremap")).unwrap_or(false));

  let keyboard = if let Some(index) = xremap_keyboard {
    keyboards.remove(index)
  } else {
    keyboards.remove(0)
  };

  let mouse = Arc::new(Mutex::new(mouse));
  let keyboard = Arc::new(Mutex::new(keyboard));

  let mut controller = Controller::try_create(mouse, keyboard).unwrap();

  process_input_events(&mut controller);
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

fn process_input_events(controller: &mut Controller) {
  let mouse_fd = controller.mouse().lock().unwrap().as_raw_fd();
  let keyboard_fd = controller.keyboard().lock().unwrap().as_raw_fd();

  let epoll_fd = epoll::create(false).expect("Failed to create epoll fd");

  let event_mouse = Event::new(Events::EPOLLIN, mouse_fd as u64);
  let event_keyboard = Event::new(Events::EPOLLIN, keyboard_fd as u64);

  epoll::ctl(epoll_fd, EPOLL_CTL_ADD, mouse_fd, event_mouse).unwrap();
  epoll::ctl(epoll_fd, EPOLL_CTL_ADD, keyboard_fd, event_keyboard).unwrap();

  let mut fd_map = std::collections::HashMap::new();

  let mouse = Arc::clone(controller.mouse());
  let keyboard = Arc::clone(controller.keyboard());

  fd_map.insert(mouse_fd, mouse);
  fd_map.insert(keyboard_fd, keyboard);

  let mut events = vec![Event::new(Events::empty(), 0); 2];

  loop {
    let num_events = epoll::wait(epoll_fd, -1, &mut events).unwrap();

    for epoll_event in events.iter().take(num_events) {
      let fd = epoll_event.data as i32;

      if let Some(device) = fd_map.get(&fd) {
        let events: Vec<_> = {
          let mut guard = device.lock().unwrap();
          guard.fetch_events().unwrap().collect()
        };

        for event in events {
          match ControllerEvent::try_from(event.destructure()) {
            Ok(controller_event) => match controller_event {
              ControllerEvent::Button {
                event,
              } => controller.handle_button_event(event),
              ControllerEvent::JoyStick {
                event,
              } => controller.handle_joystick_event(event),
            },
            Err(err) => match err {
              ControllerError::Button(e) => eprintln!("Button error: {:#?}", e),
              ControllerError::JoyStick(e) => eprintln!("Joystick error: {:#?}", e),
              ControllerError::UnsupportedEvent(e) => eprintln!("Unsupported event: {:#?}", e),
            },
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {}
