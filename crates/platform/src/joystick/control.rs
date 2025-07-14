use controller::{Axis, JoyStick, Polarity, State, Vector};

use crate::{joystick::ControllerJoyStickEvent, Controller};

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

    let vector = match joystick {
      JoyStick::Left => direction.map(Vector::from).unwrap_or_default().flipped_y(),
      JoyStick::Right => {
        let delta = f64::from(polarity);
        match axis {
          Axis::X => Vector::new(delta, 0.0),
          Axis::Y => Vector::new(0.0, delta),
        }
      }
    };

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

  fn update_left_stick_direction(&self, axis: &Axis, polarity: Polarity, state: &State) {
    let mut stick = self.left_stick.lock().unwrap();

    match axis {
      Axis::X => match polarity {
        Polarity::Negative(_) => stick.set_left(*state),
        Polarity::Positive(_) => stick.set_right(*state),
      },
      Axis::Y => match polarity {
        Polarity::Negative(_) => stick.set_down(*state),
        Polarity::Positive(_) => stick.set_up(*state),
      },
    }

    stick.update_direction();
  }
}
