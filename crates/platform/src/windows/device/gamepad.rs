use controller::{
  Axis, ButtonEvent, ControllerButton, ControllerError, ControllerEvent, JoyStick, JoyStickEvent,
  State,
};
use vigem_client::{XButtons, XGamepad};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gamepad {
  handle: XGamepad,
}

impl Default for Gamepad {
  fn default() -> Self {
    Self {
      handle: XGamepad::default(),
    }
  }
}

impl Gamepad {
  pub fn handle(&self) -> XGamepad {
    self.handle
  }

  pub fn update(&mut self, event: &ControllerEvent) -> Result<(), ControllerError> {
    use ControllerEvent::*;
    match event {
      Button(event) => self.handle_button_event(event),
      JoyStick(event) => self.handle_joystick_event(event),
    }
  }

  fn handle_button_event(&mut self, event: &ButtonEvent) -> Result<(), ControllerError> {
    use ControllerButton::*;
    use State::*;

    let button = event.button();
    let state = event.state();

    if button.eq(&L2) {
      match state {
        Pressed | Held => {
          self.handle.left_trigger = 255;
        }
        _ => {
          self.handle.left_trigger = 0;
        }
      };
      return Ok(());
    }

    if button.eq(&R2) {
      match state {
        Pressed | Held => {
          self.handle.right_trigger = 255;
        }
        _ => {
          self.handle.right_trigger = 0;
        }
      };
      return Ok(());
    }

    let button_mask = match button {
      South => XButtons::A,
      East => XButtons::B,
      West => XButtons::X,
      North => XButtons::Y,
      Up => XButtons::UP,
      Down => XButtons::DOWN,
      Left => XButtons::LEFT,
      Right => XButtons::RIGHT,
      L1 => XButtons::LB,
      R1 => XButtons::RB,
      L3 => XButtons::LTHUMB,
      R3 => XButtons::RTHUMB,
      Start => XButtons::START,
      Select => XButtons::BACK,
      Forward | Backward | Starboard | Port => 0,
      L2 | R2 => 0,
    };

    match state {
      Pressed | Held => {
        self.handle.buttons.raw |= button_mask;
      }
      _ => {
        self.handle.buttons.raw &= !button_mask;
      }
    };

    println!("Updated gamepad: {:#?}", self.handle);
    Ok(())
  }

  fn handle_joystick_event(&mut self, event: &JoyStickEvent) -> Result<(), ControllerError> {
    use Axis::*;
    use JoyStick::*;

    let joystick = event.joystick();
    let axis = event.axis();
    let polarity = event.polarity();

    match joystick {
      Left => match axis {
        X => self.handle.thumb_lx = polarity.into(),
        Y => self.handle.thumb_ly = polarity.into(),
      },
      Right => match axis {
        X => self.handle.thumb_rx = polarity.into(),
        Y => self.handle.thumb_ry = polarity.into(),
      },
    };

    Ok(())
  }
}
