mod button;
mod error;
mod event;
mod joystick;

pub use button::*;
pub use error::*;
pub use event::*;
pub use joystick::*;

use settings::SETTINGS;

use std::sync::{Arc, Mutex};

pub trait ControllerEventEmitter: Send + Sync {
  fn emit(&mut self, events: &[ControllerEvent]) -> Result<(), ControllerError>;
}

pub struct VirtualController {
  emitter: Box<dyn ControllerEventEmitter>,
  left_stick: Arc<Mutex<JoyStickState>>,
  right_stick: Arc<Mutex<JoyStickState>>,
}

impl VirtualController {
  pub fn new(
    emitter: Box<dyn ControllerEventEmitter>,
    left_stick: Arc<Mutex<JoyStickState>>,
    right_stick: Arc<Mutex<JoyStickState>>,
  ) -> Self {
    Self {
      emitter,
      left_stick,
      right_stick,
    }
  }

  pub fn emit(&mut self, events: &[ControllerEvent]) -> Result<(), ControllerError> {
    self.emitter.emit(events)
  }

  pub fn left_stick(&self) -> &Mutex<JoyStickState> {
    &self.left_stick
  }

  pub fn right_stick(&self) -> &Mutex<JoyStickState> {
    &self.right_stick
  }

  pub fn left_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>> {
    &mut self.left_stick
  }

  pub fn right_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>> {
    &mut self.right_stick
  }

  pub fn handle_event(&mut self, event: ControllerEvent) -> Result<(), ControllerError> {
    match event {
      ControllerEvent::Button(event) => self.handle_button_event(event),
      ControllerEvent::JoyStick(event) => self.handle_joystick_event(event),
    }
  }

  pub fn handle_button_event(&mut self, event: ButtonEvent) -> Result<(), ControllerError> {
    self.emit(&[ControllerEvent::from(event)])
  }

  pub fn handle_joystick_event(&mut self, event: JoyStickEvent) -> Result<(), ControllerError> {
    let joystick = event.joystick();
    let axis = event.axis();
    let polarity = event.polarity();
    let state = event.state();

    if *joystick == JoyStick::Left {
      self.update_left_stick_direction(axis, polarity, &state);
    }

    let direction = self.left_stick.lock().unwrap().direction();
    let vector = Vector::from((axis, polarity, joystick, direction));

    if *joystick == JoyStick::Right {
      let vector = self.right_stick.lock().unwrap().micro(vector);
      self.move_right_stick(vector)
    } else {
      let vector = {
        let mut stick = self.left_stick.lock().unwrap();
        stick.tilt(vector)
      };
      self.move_left_stick(vector, None)
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

  fn move_left_stick(
    &mut self,
    vector: Vector,
    direction: Option<Direction>,
  ) -> Result<(), ControllerError> {
    let (x, y) = if let Some(direction) = direction {
      if direction == Direction::North {
        (0.0, -vector.dy() * 2.0)
      } else {
        (vector.dx(), -vector.dy())
      }
    } else {
      (vector.dx(), -vector.dy())
    };

    self.emit(&[
      Self::get_stick_event(JoyStick::Left, Axis::X, x),
      Self::get_stick_event(JoyStick::Left, Axis::Y, y),
    ])
  }

  fn move_right_stick(&mut self, vector: Vector) -> Result<(), ControllerError> {
    self.emit(&[
      Self::get_stick_event(JoyStick::Right, Axis::X, vector.dx()),
      Self::get_stick_event(JoyStick::Right, Axis::Y, vector.dy()),
    ])
  }

  fn get_stick_event(stick: JoyStick, axis: Axis, value: f64) -> ControllerEvent {
    let polarity = Polarity::try_from(value).unwrap();
    let state = State::Pressed;
    let joystick_event = JoyStickEvent::new(stick, axis, polarity, state);
    ControllerEvent::from(joystick_event)
  }

  fn monitor_left_stick(controller: Arc<Mutex<Self>>) -> ! {
    loop {
      {
        controller.lock().unwrap().handle_left_stick().unwrap();
      }
      std::thread::sleep(std::time::Duration::from_millis(1));
    }
  }

  fn handle_left_stick(&mut self) -> Result<(), ControllerError> {
    let maybe_direction = {
      let stick_lock = self.left_stick_mut();
      let stick = stick_lock.lock().unwrap();
      stick.direction()
    };

    if let Some(direction) = maybe_direction {
      let vector = Vector::from(direction) * settings::LEFT_STICK_SENSITIVITY;

      let vector = {
        let mut stick = self.left_stick_mut().lock().unwrap();
        stick.tilt(vector)
      };

      self.move_left_stick(vector, Some(direction))
    } else {
      self.center_left_stick()
    }
  }

  fn center_left_stick(&mut self) -> Result<(), ControllerError> {
    self.move_left_stick(Vector::default(), None)
  }

  fn center_right_stick(&mut self) -> Result<(), ControllerError> {
    self.move_right_stick(Vector::default())
  }

  fn monitor_right_stick(controller: Arc<Mutex<Self>>) -> ! {
    loop {
      {
        controller.lock().unwrap().handle_right_stick().unwrap();
      }
      std::thread::sleep(SETTINGS.tickrate());
    }
  }

  fn handle_right_stick(&mut self) -> Result<(), ControllerError> {
    let left_stick_direction = { self.left_stick.lock().unwrap().direction() };

    if self.right_stick_mut().lock().unwrap().handle_idle(left_stick_direction) {
      return self.center_right_stick();
    }

    Ok(())
  }
}
