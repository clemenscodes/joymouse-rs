mod controller;
mod mouse;

use crate::controller::Controller;
use std::sync::{Arc, Mutex};

fn main() {
  let mouse = Arc::new(Mutex::new(Controller::init_mouse()));
  let keyboard = Arc::new(Mutex::new(Controller::init_keyboard()));
  let controller = Arc::new(Mutex::new(Controller::try_create().unwrap()));

  let controller_for_input = Arc::clone(&controller);
  let input_thread = std::thread::spawn(move || {
    Controller::process_input_events(mouse, keyboard, controller_for_input);
  });

  let stick_thread = std::thread::spawn(move || {
    Controller::monitor_sticks(Arc::clone(&controller));
  });

  println!("Started JoyMouse 🎮🐭");

  input_thread.join().unwrap();
  stick_thread.join().unwrap();
}
