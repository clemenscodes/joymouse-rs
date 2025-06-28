mod controller;

use crate::controller::{Controller, ControllerError, ControllerEvent};

use std::os::fd::AsRawFd;

use epoll::{ControlOptions::EPOLL_CTL_ADD, Event, Events};
use evdev::{Device, EventType, KeyCode, MiscCode, RelativeAxisCode};

fn main() {
  let mut mice = find_mice();

  let mut keyboards = find_keyboards();

  let mouse = mice.first_mut().unwrap();

  let xremap_keyboard = keyboards.iter_mut().find(|keyboard| {
    if let Some(name) = keyboard.name() {
      return name.contains("xremap");
    }
    false
  });

  let keyboard = if let Some(keyboard) = xremap_keyboard {
    keyboard
  } else {
    keyboards.first_mut().unwrap()
  };

  let mut controller = Controller::try_create().unwrap();

  process_input_events(mouse, keyboard, &mut controller);
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

fn process_input_events(mouse: &mut Device, keyboard: &mut Device, controller: &mut Controller) {
  println!(
    "Converting mouse events from {:?} and keyboard events from {:#?} to events for {:#?}",
    mouse.name().unwrap(),
    keyboard.name().unwrap(),
    controller
  );

  let mouse_file_descriptor = mouse.as_raw_fd();
  let keyboard_file_descriptor = keyboard.as_raw_fd();

  let epoll_file_descriptor = match epoll::create(false) {
    Ok(file_descriptor) => file_descriptor,
    Err(_) => {
      eprintln!("Failed to create epoll file descriptor");
      std::process::exit(1);
    }
  };

  let event_mouse = Event::new(Events::EPOLLIN, mouse_file_descriptor as u64);
  let event_keyboard = Event::new(Events::EPOLLIN, keyboard_file_descriptor as u64);

  epoll::ctl(epoll_file_descriptor, EPOLL_CTL_ADD, mouse_file_descriptor, event_mouse).unwrap();
  epoll::ctl(epoll_file_descriptor, EPOLL_CTL_ADD, keyboard_file_descriptor, event_keyboard).unwrap();

  let mut file_descriptor_map = std::collections::HashMap::new();

  file_descriptor_map.insert(mouse_file_descriptor, &mut *mouse);
  file_descriptor_map.insert(keyboard_file_descriptor, &mut *keyboard);

  let mut events = vec![Event::new(Events::empty(), 0); 2];

  loop {
    let num_events = epoll::wait(epoll_file_descriptor, -1, &mut events).unwrap();
    for event in events.iter().take(num_events) {
      let file_descriptor = event.data as i32;
      if let Some(device) = file_descriptor_map.get_mut(&file_descriptor) {
        for event in device.fetch_events().unwrap() {
          match ControllerEvent::try_from(event.destructure()) {
            Ok(event) => match event {
              ControllerEvent::Button {
                ref event,
              } => controller.handle_button_event(event),
              ControllerEvent::JoyStick {
                ref event,
              } => controller.handle_joystick_event(event),
            },
            Err(err) => match err {
              ControllerError::Button(button_error) => {
                eprintln!("Failed to convert event to a controller button event: {:#?}", button_error);
              }
              ControllerError::JoyStick(joy_stick_error) => {
                eprintln!("Failed to convert event to a joystick event: {:#?}", joy_stick_error);
              }
              _ => (),
            },
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {}
