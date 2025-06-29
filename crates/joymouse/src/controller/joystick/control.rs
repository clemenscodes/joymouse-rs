use evdev::{AbsoluteAxisCode, EventType, InputEvent};

use crate::controller::{
  Controller,
  joystick::{ControllerJoyStickEvent, JoyStick, axis::JoyStickAxis, polarity::Polarity, vector::Vector},
  state::State,
};

impl Controller {
  pub fn handle_joystick_event(&mut self, event: ControllerJoyStickEvent, original: InputEvent) {
    let code = AbsoluteAxisCode::from(&event);
    let joystick = event.joystick();
    let axis = event.axis();
    let polarity = event.polarity();
    let state = event.state();

    if *joystick == JoyStick::Left {
      self.update_left_stick_direction(axis, polarity, state);
    } else {
      self.mouse_mut().emit(original);
    }

    let direction = self.left_stick.lock().unwrap().direction();
    let vector = Vector::from((axis, polarity, joystick, direction));

    if *joystick == JoyStick::Right {
      let position = self.right_stick.lock().unwrap().tilt(vector);
      let virtual_event = InputEvent::new(EventType::ABSOLUTE.0, code.0, position);
      let events = vec![virtual_event];
      self.virtual_device.emit(&events).unwrap();
    } else {
      self.left_stick.lock().unwrap().tilt(vector);
      let stick = self.left_stick.lock().unwrap();
      let events = vec![
        InputEvent::new(EventType::ABSOLUTE.0, AbsoluteAxisCode::ABS_X.0, stick.x()),
        InputEvent::new(EventType::ABSOLUTE.0, AbsoluteAxisCode::ABS_Y.0, stick.y()),
      ];
      self.virtual_device.emit(&events).unwrap();
    }
  }

  fn update_left_stick_direction(&self, axis: &JoyStickAxis, polarity: Polarity, state: &State) {
    let mut stick = self.left_stick.lock().unwrap();

    match axis {
      JoyStickAxis::X => match polarity {
        Polarity::Negative(_) => stick.set_left(*state),
        Polarity::Positive(_) => stick.set_right(*state),
      },
      JoyStickAxis::Y => match polarity {
        Polarity::Negative(_) => stick.set_down(*state),
        Polarity::Positive(_) => stick.set_up(*state),
      },
    }

    stick.update_direction();
  }
}
