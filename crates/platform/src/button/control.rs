use controller::ButtonEvent;

use crate::{button::from_button_event_for_input_event, Controller};

impl Controller {
  pub fn handle_button_event(&mut self, event: ButtonEvent) {
    let button_event = from_button_event_for_input_event(event);
    self.emit_button_events(&[button_event]);
  }
}
