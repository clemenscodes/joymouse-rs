mod button;
mod joystick;

use std::sync::{Arc, Mutex};

pub use button::*;
pub use joystick::*;

pub trait ControllerEventEmitter: Send + Sync {}

pub struct VirtualController {
  emitter: Box<dyn ControllerEventEmitter>,
  left_stick: Arc<Mutex<JoyStickState>>,
  right_stick: Arc<Mutex<JoyStickState>>,
}
