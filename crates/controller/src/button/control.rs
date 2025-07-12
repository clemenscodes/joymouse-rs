use evdev::InputEvent;

use crate::{button::ControllerButtonEvent, Controller};

impl Controller {
  pub fn handle_button_event(&mut self, event: ControllerButtonEvent) {
    let virtual_event = InputEvent::from(event);
    let events = vec![virtual_event];
    self.emit_events(&events);
  }
}
