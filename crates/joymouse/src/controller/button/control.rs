use evdev::{InputEvent, KeyCode};

use crate::controller::{Controller, button::ControllerButtonEvent};

impl Controller {
  pub fn handle_button_event(&mut self, event: ControllerButtonEvent, original: InputEvent) {
    let virtual_event = InputEvent::from(event);
    let events = vec![virtual_event];
    self.virtual_device.emit(&events).unwrap();
    if let evdev::EventSummary::Key(_, key_code, _) = original.destructure() {
      match key_code {
        KeyCode::BTN_LEFT => self.mouse_mut().emit(original), // TODO: only when not fullscreened?
        KeyCode::BTN_RIGHT => self.mouse_mut().emit(original),
        _ => (),
      };
    }
  }
}
