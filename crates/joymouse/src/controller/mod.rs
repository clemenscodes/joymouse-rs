mod button;
mod event;
mod joystick;
mod state;

use crate::controller::{button::ControllerButtonEvent, joystick::ControllerJoyStickEvent};

use evdev::{
  AbsInfo, AbsoluteAxisCode, AttributeSet, BusType, InputId, KeyCode, UinputAbsSetup, uinput::VirtualDevice,
};

#[derive(Debug)]
pub struct Controller {
  device: VirtualDevice,
}

impl Controller {
  pub fn try_create() -> Result<Self, Box<dyn std::error::Error>> {
    let builder = VirtualDevice::builder()?;

    let name = "JoyMouse";

    let vendor = 0x1234;
    let product = 0x5678;
    let version = 0x0100;
    let input_id = InputId::new(BusType::BUS_USB, vendor, product, version);

    let mut button_set = AttributeSet::<KeyCode>::new();

    let buttons = [
      KeyCode::BTN_SOUTH,
      KeyCode::BTN_SOUTH,
      KeyCode::BTN_EAST,
      KeyCode::BTN_NORTH,
      KeyCode::BTN_WEST,
      KeyCode::BTN_TL,
      KeyCode::BTN_TR,
      KeyCode::BTN_TL2,
      KeyCode::BTN_TR2,
      KeyCode::BTN_START,
      KeyCode::BTN_SELECT,
      KeyCode::BTN_THUMBL,
      KeyCode::BTN_THUMBR,
      KeyCode::BTN_DPAD_UP,
      KeyCode::BTN_DPAD_DOWN,
      KeyCode::BTN_DPAD_LEFT,
      KeyCode::BTN_DPAD_RIGHT,
    ];

    for button in buttons {
      button_set.insert(button);
    }

    let axis_info = AbsInfo::new(0, -32768, 32767, 0, 4096, 1);
    let x_axis = UinputAbsSetup::new(AbsoluteAxisCode::ABS_X, axis_info);
    let y_axis = UinputAbsSetup::new(AbsoluteAxisCode::ABS_Y, axis_info);
    let rx_axis = UinputAbsSetup::new(AbsoluteAxisCode::ABS_RX, axis_info);
    let ry_axis = UinputAbsSetup::new(AbsoluteAxisCode::ABS_RY, axis_info);

    let device = builder
      .name(&name)
      .input_id(input_id)
      .with_keys(&button_set)?
      .with_absolute_axis(&x_axis)?
      .with_absolute_axis(&y_axis)?
      .with_absolute_axis(&rx_axis)?
      .with_absolute_axis(&ry_axis)?
      .build()?;

    Ok(Self {
      device,
    })
  }

  pub fn handle_button_event(&mut self, event: &ControllerButtonEvent) {
    println!("Handling controller button event: {:#?}", event);
  }

  pub fn handle_joystick_event(&mut self, event: &ControllerJoyStickEvent) {
    println!("Handling controller joystick event: {:#?}", event);
  }
}

pub use event::{ControllerError, ControllerEvent};
