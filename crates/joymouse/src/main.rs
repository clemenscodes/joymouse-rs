mod controller;
mod mouse;

use crate::controller::Controller;
use std::sync::{Arc, Mutex};

fn main() {
  let mouse = Arc::new(Mutex::new(Controller::init_mouse()));
  let keyboard = Arc::new(Mutex::new(Controller::init_keyboard()));
  let controller = Arc::new(Mutex::new(Controller::try_create().unwrap()));

  let left_stick = Arc::clone(&controller);
  std::thread::spawn(move || Controller::monitor_left_stick(left_stick));

  let right_stick = Arc::clone(&controller);
  std::thread::spawn(move || Controller::monitor_right_stick(right_stick));

  let io = std::thread::spawn(move || Controller::monitor_io(mouse, keyboard, controller));

  println!("Started JoyMouse 🎮🐭");

  io.join().unwrap();
}
