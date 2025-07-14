use evdev::InputEvent;

use crate::{button::event::ControllerButtonEvent, Controller};

impl Controller {
  pub fn handle_button_event(&mut self, event: ControllerButtonEvent) {
    self.emit_button_events(&[InputEvent::from(event)]);
  }
}
