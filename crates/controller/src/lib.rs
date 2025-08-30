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

pub trait PlatformControllerManager: VirtualController + Sized + 'static {
  type Ops: PlatformControllerOps;

  fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting JoyMouse 🎮🐭");
    let controller = Arc::new(Mutex::new(Self::try_create()?));

    let signal_handler = Arc::clone(&controller);
    let _ = ctrlc::set_handler(move || {
      println!("Stopping JoyMouse 🎮🐭");
      signal_handler.lock().unwrap().disconnect().unwrap();
      println!("Stopped JoyMouse 🎮🐭");
      std::process::exit(0);
    });

    let io_controller = Arc::clone(&controller);
    let io = std::thread::spawn(move || {
      let mouse = Self::Ops::init_mouse();
      let keyboard = Self::Ops::init_keyboard();
      Self::Ops::monitor_io(mouse, keyboard, io_controller);
    });

    let left_stick = Arc::clone(&controller);
    std::thread::spawn(move || Self::monitor_left_stick(left_stick));

    let right_stick = Arc::clone(&controller);
    std::thread::spawn(move || Self::monitor_right_stick(right_stick));

    println!("Started JoyMouse 🎮🐭");

    io.join().unwrap();
    Ok(())
  }

  fn try_create() -> Result<Self, Box<dyn std::error::Error>>;
}

pub trait PlatformControllerOps {
  type VirtualDevice;
  type PhysicalDevice;

  fn create_virtual_controller() -> Result<Self::VirtualDevice, Box<dyn std::error::Error>>;
  fn init_mouse() -> Self::PhysicalDevice;
  fn init_keyboard() -> Self::PhysicalDevice;
  fn monitor_io(
    mouse: Self::PhysicalDevice,
    keyboard: Self::PhysicalDevice,
    controller: Arc<Mutex<dyn VirtualControllerCore>>,
  ) -> !;
}

pub trait VirtualControllerCore: Send + Sync {
  fn handle_event(&mut self, event: ControllerEvent) -> Result<(), ControllerError>;
  fn disconnect(&mut self) -> Result<(), ControllerError>;
}

impl<T: VirtualController> VirtualControllerCore for T {
  fn handle_event(&mut self, event: ControllerEvent) -> Result<(), ControllerError> {
    VirtualController::handle_event(self, event)
  }

  fn disconnect(&mut self) -> Result<(), ControllerError> {
    self.disconnect()
  }
}

pub trait VirtualController: ControllerEventEmitter {
  fn left_stick(&self) -> &Mutex<JoyStickState>;

  fn right_stick(&self) -> &Mutex<JoyStickState>;

  fn left_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>>;

  fn right_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>>;

  fn handle_event(&mut self, event: ControllerEvent) -> Result<(), ControllerError> {
    match event {
      ControllerEvent::Button(e) => self.handle_button_event(e),
      ControllerEvent::JoyStick(e) => self.handle_joystick_event(e),
    }
  }

  fn handle_button_event(&mut self, event: ButtonEvent) -> Result<(), ControllerError> {
    self.emit(&[ControllerEvent::from(event)])
  }

  fn handle_joystick_event(&mut self, event: JoyStickEvent) -> Result<(), ControllerError> {
    let joystick = event.joystick();
    let axis = event.axis();
    let polarity = event.polarity();
    let state = event.state();

    if *joystick == JoyStick::Left {
      self.update_left_stick_direction(axis, &polarity, &state);
    }

    let direction = { self.left_stick().lock().unwrap().direction() };
    let vector = Vector::from((axis, polarity, joystick, direction));

    if *joystick == JoyStick::Right {
      let vector = { self.right_stick().lock().unwrap().micro(vector) };
      self.move_right_stick(vector)
    } else {
      let vector = { self.left_stick().lock().unwrap().tilt(vector) };
      self.move_left_stick(vector, None)
    }
  }

  fn update_left_stick_direction(&self, axis: &Axis, polarity: &Polarity, state: &State) {
    let mut stick = self.left_stick().lock().unwrap();

    match axis {
      Axis::X => match polarity {
        Polarity::Negative(_) => stick.set_left(*state),
        Polarity::Positive(_) => stick.set_right(*state),
        Polarity::Neutral => {}
      },
      Axis::Y => match polarity {
        Polarity::Negative(_) => stick.set_down(*state),
        Polarity::Positive(_) => stick.set_up(*state),
        Polarity::Neutral => {}
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

  fn center_left_stick(&mut self) -> Result<(), ControllerError> {
    let is_centered = { self.left_stick().lock().unwrap().is_centered() };
    if !is_centered {
      self.left_stick().lock().unwrap().recenter();
      self.emit(&[
        ControllerEvent::from(JoyStickEvent::new(
          JoyStick::Left,
          Axis::X,
          Polarity::Neutral,
          State::Released,
        )),
        ControllerEvent::from(JoyStickEvent::new(
          JoyStick::Left,
          Axis::Y,
          Polarity::Neutral,
          State::Released,
        )),
      ])?;
    }
    Ok(())
  }

  fn center_right_stick(&mut self) -> Result<(), ControllerError> {
    self.emit(&[
      ControllerEvent::from(JoyStickEvent::new(
        JoyStick::Right,
        Axis::X,
        Polarity::Neutral,
        State::Released,
      )),
      ControllerEvent::from(JoyStickEvent::new(
        JoyStick::Right,
        Axis::Y,
        Polarity::Neutral,
        State::Released,
      )),
    ])
  }

  fn monitor_left_stick(controller: Arc<Mutex<Self>>) -> !
  where
    Self: Sized,
  {
    loop {
      controller.lock().unwrap().handle_left_stick().unwrap();
      std::thread::sleep(std::time::Duration::from_millis(1));
    }
  }

  fn monitor_right_stick(controller: Arc<Mutex<Self>>) -> !
  where
    Self: Sized,
  {
    loop {
      controller.lock().unwrap().handle_right_stick().unwrap();
      std::thread::sleep(SETTINGS.tickrate());
    }
  }

  fn handle_left_stick(&mut self) -> Result<(), ControllerError> {
    let maybe_direction = { self.left_stick_mut().lock().unwrap().direction() };
    if let Some(direction) = maybe_direction {
      let vector = Vector::from(direction) * settings::LEFT_STICK_SENSITIVITY;
      let vector = { self.left_stick_mut().lock().unwrap().tilt(vector) };
      self.move_left_stick(vector, Some(direction))
    } else {
      self.center_left_stick()
    }
  }

  fn handle_right_stick(&mut self) -> Result<(), ControllerError> {
    let left_stick_direction = self.left_stick().lock().unwrap().direction();
    if self.right_stick().lock().unwrap().handle_idle(left_stick_direction) {
      self.center_right_stick()
    } else {
      Ok(())
    }
  }

  fn get_stick_event(stick: JoyStick, axis: Axis, value: f64) -> ControllerEvent {
    let polarity = Polarity::from(value);
    let state = State::Pressed;
    let joystick_event = JoyStickEvent::new(stick, axis, polarity, state);
    ControllerEvent::from(joystick_event)
  }
}

pub trait ControllerEventEmitter: Send + Sync {
  fn emit(&mut self, events: &[ControllerEvent]) -> Result<(), ControllerError>;
  fn disconnect(&mut self) -> Result<(), ControllerError>;
}
