mod controller;
mod mouse;

use crate::controller::Controller;

use std::sync::{Arc, Mutex};

fn main() {
  let mut mice = Controller::find_mice();
  let mut keyboards = Controller::find_keyboards();

  let mouse = Arc::new(Mutex::new(mice.remove(0)));
  let xremap_keyboard =
    keyboards.iter().position(|keyboard| keyboard.name().is_some_and(|name| name.contains("xremap")));

  let keyboard = Arc::new(Mutex::new(if let Some(index) = xremap_keyboard {
    keyboards.remove(index)
  } else {
    keyboards.remove(0)
  }));

  let mouse = Arc::clone(&mouse);
  let keyboard = Arc::clone(&keyboard);
  let controller = Arc::new(Mutex::new(Controller::try_create().unwrap()));
  let stick_controller = Arc::clone(&controller);
  let input_controller = Arc::clone(&controller);

  mouse.lock().unwrap().grab().unwrap();

  std::thread::spawn(move || {
    stick_controller.lock().unwrap().monitor_sticks();
  });

  let process = std::thread::spawn(move || {
    input_controller.lock().unwrap().process_input_events(mouse, keyboard);
  });

  println!("Started JoyMouse 🎮🐭");

  process.join().unwrap();
}
