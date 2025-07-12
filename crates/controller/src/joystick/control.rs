use crate::{
  joystick::{
    axis::JoyStickAxis, polarity::Polarity, vector::Vector, ControllerJoyStickEvent, JoyStick,
  },
  state::State,
  Controller,
};

impl Controller {
  pub fn handle_joystick_event(&mut self, event: ControllerJoyStickEvent) {
    let joystick = event.joystick();
    let axis = event.axis();
    let polarity = event.polarity();
    let state = event.state();

    if *joystick == JoyStick::Left {
      self.update_left_stick_direction(axis, polarity, state);
    }

    let direction = self.left_stick.lock().unwrap().direction();
    let vector = Vector::from((axis, polarity, joystick, direction));

    if *joystick == JoyStick::Right {
      let vector = self.right_stick.lock().unwrap().micro(vector);
      self.move_right_stick(vector);
    } else {
      let vector = {
        let mut stick = self.left_stick.lock().unwrap();
        stick.tilt(vector)
      };
      self.move_left_stick(vector, None);
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
