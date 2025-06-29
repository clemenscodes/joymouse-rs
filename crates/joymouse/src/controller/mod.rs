mod button;
mod error;
mod event;
mod joystick;
mod settings;

use std::sync::{Arc, Mutex, mpsc};

use crate::{controller::joystick::JoyStickState, mouse::Mouse};

use evdev::{
  AbsInfo, AbsoluteAxisCode, AttributeSet, BusType, InputEvent, InputId, KeyCode, UinputAbsSetup, uinput::VirtualDevice,
};

#[derive(Debug)]
pub struct Controller {
  mouse: Mouse,
  virtual_device: VirtualDevice,
  left_stick: Arc<Mutex<JoyStickState>>,
  right_stick: Arc<Mutex<JoyStickState>>,
  left_idle_cancel: Option<mpsc::Sender<()>>,
  right_idle_cancel: Option<mpsc::Sender<()>>,
}

impl Controller {
  pub fn try_create(mouse: Mouse) -> Result<Self, Box<dyn std::error::Error>> {
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

    let virtual_device = builder
      .name(&name)
      .input_id(input_id)
      .with_keys(&button_set)?
      .with_absolute_axis(&x_axis)?
      .with_absolute_axis(&y_axis)?
      .with_absolute_axis(&rx_axis)?
      .with_absolute_axis(&ry_axis)?
      .build()?;

    Ok(Self {
      mouse,
      virtual_device,
      left_stick: Arc::new(Mutex::new(JoyStickState::default())),
      right_stick: Arc::new(Mutex::new(JoyStickState::default())),
      left_idle_cancel: None,
      right_idle_cancel: None,
    })
  }

  pub fn handle_event(&mut self, event: ControllerEvent, original: InputEvent) {
    match event {
      ControllerEvent::Button(event) => self.handle_button_event(event, original),
      ControllerEvent::JoyStick(event) => self.handle_joystick_event(event, original),
    }
  }

  pub fn mouse_mut(&mut self) -> &mut Mouse {
    &mut self.mouse
  }
}

pub use event::ControllerEvent;
