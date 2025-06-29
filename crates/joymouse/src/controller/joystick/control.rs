use evdev::{AbsoluteAxisCode, EventType, InputEvent};

use crate::controller::{
  Controller,
  joystick::{ControllerJoyStickEvent, JoyStick, axis::JoyStickAxis},
};

impl Controller {
  pub fn handle_joystick_event(&mut self, event: ControllerJoyStickEvent, original: InputEvent) {
    println!("Processing joystick event: {:#?}", event);

    let code = AbsoluteAxisCode::from(&event);

    let delta = event.value();

    let joystick = event.joystick();

    let (dx, dy) = match event.axis() {
      JoyStickAxis::X => (delta, 0),
      JoyStickAxis::Y => match event.joystick() {
        JoyStick::Left => (0, -delta),
        JoyStick::Right => (0, delta),
      },
    };

    let value = match event.joystick() {
      JoyStick::Left => {
        let stick = self.left_stick.clone();
        stick.lock().unwrap().tilt(dx, dy)
      }
      JoyStick::Right => {
        let stick = self.right_stick.clone();
        stick.lock().unwrap().tilt(dx, dy)
      }
    };

    let virtual_event = InputEvent::new(EventType::ABSOLUTE.0, code.0, value);
    let events = vec![virtual_event];
    self.virtual_device.emit(&events).unwrap();

    if *joystick == JoyStick::Right {
      self.mouse_mut().emit(original);
    }
  }
}
