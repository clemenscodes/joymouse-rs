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

    let vector = Self::vector(axis, polarity, joystick);
    let position = self.position(joystick, vector);
    let virtual_event = InputEvent::new(EventType::ABSOLUTE.0, code.0, position);
    let events = vec![virtual_event];
    self.virtual_device.emit(&events).unwrap();
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

  fn vector(axis: &JoyStickAxis, polarity: Polarity, joystick: &JoyStick) -> Vector {
    let delta = i32::from(polarity);
    match axis {
      JoyStickAxis::X => Vector::new(delta, 0),
      JoyStickAxis::Y => match joystick {
        JoyStick::Left => Vector::new(0, -delta),
        JoyStick::Right => Vector::new(0, delta),
      },
    }
  }

  fn position(&mut self, joystick: &JoyStick, vector: Vector) -> i32 {
    match joystick {
      JoyStick::Left => {
        let stick = self.left_stick.clone();
        stick.lock().unwrap().tilt(vector)
      }
      JoyStick::Right => {
        let stick = self.right_stick.clone();
        stick.lock().unwrap().tilt(vector)
      }
    }
  }
}
