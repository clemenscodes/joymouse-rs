mod device;

use crate::windows::device::VirtualDevice;

use bindings::KEYBOARD_BUTTON_MAP;
use controller::{
  ButtonEvent, ControllerError, ControllerEvent, ControllerEventEmitter, JoyStickState,
  PlatformControllerManager, PlatformControllerOps, State, VirtualController,
  VirtualControllerCore,
};
use io::{AlphabeticKey, ArrowKey, FunctionKey, Key, ModifierKey, NumericKey, SystemKey};

use std::{
  sync::{Arc, Mutex},
  time::Duration,
};

use device_query::{DeviceEvents, DeviceEventsHandler, Keycode, MouseButton};

pub struct Controller {
  virtual_device: <WindowsOps as PlatformControllerOps>::VirtualDevice,
  left_stick: Arc<Mutex<JoyStickState>>,
  right_stick: Arc<Mutex<JoyStickState>>,
}

impl ControllerEventEmitter for Controller {
  fn emit(&mut self, events: &[ControllerEvent]) -> Result<(), ControllerError> {
    self.virtual_device.emit(events)
  }

  fn disconnect(&mut self) -> Result<(), ControllerError> {
    self.virtual_device.disconnect()
  }
}

impl VirtualController for Controller {
  fn left_stick(&self) -> &Mutex<JoyStickState> {
    &self.left_stick
  }

  fn left_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>> {
    &mut self.left_stick
  }

  fn right_stick(&self) -> &Mutex<JoyStickState> {
    &self.right_stick
  }

  fn right_stick_mut(&mut self) -> &mut Arc<Mutex<JoyStickState>> {
    &mut self.right_stick
  }
}

pub struct WindowsOps;

impl PlatformControllerOps for WindowsOps {
  type VirtualDevice = VirtualDevice;
  type PhysicalDevice = ();

  fn init_mouse() -> Self::PhysicalDevice {
    ()
  }

  fn init_keyboard() -> Self::PhysicalDevice {
    ()
  }

  fn create_virtual_controller() -> Result<Self::VirtualDevice, Box<dyn std::error::Error>> {
    let controller = Self::VirtualDevice::default();
    Ok(controller)
  }

  fn monitor_io(
    _mouse: Self::PhysicalDevice,
    _keyboard: Self::PhysicalDevice,
    controller: Arc<Mutex<dyn VirtualControllerCore>>,
  ) -> ! {
    use State::*;

    let handler = DeviceEventsHandler::new(Duration::from_millis(10))
      .expect("Failed to create DeviceEventsHandler");

    let _g_key_down_controller = Arc::clone(&controller);
    let _g_key_down = handler.on_key_down(move |key: &Keycode| {
      if let Some(button) = &map_key(key) {
        if let Some(button) = KEYBOARD_BUTTON_MAP.get(button) {
          let button_event = ButtonEvent::new(*button, Pressed);
          let controller_event = ControllerEvent::from(button_event);
          _g_key_down_controller.lock().unwrap().handle_event(controller_event).unwrap();
        }
      }
    });

    let _g_key_up_controller = Arc::clone(&controller);
    let _g_key_up = handler.on_key_up(move |key: &Keycode| {
      if let Some(button) = &map_key(key) {
        if let Some(button) = KEYBOARD_BUTTON_MAP.get(button) {
          let button_event = ButtonEvent::new(*button, Released);
          let controller_event = ControllerEvent::from(button_event);
          _g_key_up_controller.lock().unwrap().handle_event(controller_event).unwrap();
        }
      }
    });

    let _g_mouse_move = handler.on_mouse_move(|_pos: &(i32, i32)| {});
    let _g_mouse_down = handler.on_mouse_down(|_btn: &MouseButton| {});
    let _g_mouse_up = handler.on_mouse_up(|_btn: &MouseButton| {});

    loop {
      std::thread::sleep(Duration::from_secs(1));
    }
  }
}

fn map_key(key: &Keycode) -> Option<Key> {
  use Key::*;
  use Keycode::*;
  use NumericKey::*;

  Some(match key {
    Key0 => Numeric(Num0),
    Key1 => Numeric(Num1),
    Key2 => Numeric(Num2),
    Key3 => Numeric(Num3),
    Key4 => Numeric(Num4),
    Key5 => Numeric(Num5),
    Key6 => Numeric(Num6),
    Key7 => Numeric(Num7),
    Key8 => Numeric(Num8),
    Key9 => Numeric(Num9),
    A => Alphabetic(AlphabeticKey::A),
    B => Alphabetic(AlphabeticKey::B),
    C => Alphabetic(AlphabeticKey::C),
    D => Alphabetic(AlphabeticKey::D),
    E => Alphabetic(AlphabeticKey::E),
    F => Alphabetic(AlphabeticKey::F),
    G => Alphabetic(AlphabeticKey::G),
    H => Alphabetic(AlphabeticKey::H),
    I => Alphabetic(AlphabeticKey::I),
    J => Alphabetic(AlphabeticKey::J),
    K => Alphabetic(AlphabeticKey::K),
    L => Alphabetic(AlphabeticKey::L),
    M => Alphabetic(AlphabeticKey::M),
    N => Alphabetic(AlphabeticKey::N),
    O => Alphabetic(AlphabeticKey::O),
    P => Alphabetic(AlphabeticKey::P),
    Q => Alphabetic(AlphabeticKey::Q),
    R => Alphabetic(AlphabeticKey::R),
    S => Alphabetic(AlphabeticKey::S),
    T => Alphabetic(AlphabeticKey::T),
    U => Alphabetic(AlphabeticKey::U),
    V => Alphabetic(AlphabeticKey::V),
    W => Alphabetic(AlphabeticKey::W),
    X => Alphabetic(AlphabeticKey::X),
    Y => Alphabetic(AlphabeticKey::Y),
    Z => Alphabetic(AlphabeticKey::Z),
    F1 => Function(FunctionKey::F1),
    F2 => Function(FunctionKey::F2),
    F3 => Function(FunctionKey::F3),
    F4 => Function(FunctionKey::F4),
    F5 => Function(FunctionKey::F5),
    F6 => Function(FunctionKey::F6),
    F7 => Function(FunctionKey::F7),
    F8 => Function(FunctionKey::F8),
    F9 => Function(FunctionKey::F9),
    F10 => Function(FunctionKey::F10),
    F11 => Function(FunctionKey::F11),
    F12 => Function(FunctionKey::F12),
    Escape => Modifier(ModifierKey::Escape),
    Space => System(SystemKey::Space),
    LControl => Modifier(ModifierKey::LeftCtrl),
    RControl => Modifier(ModifierKey::RightCtrl),
    LShift => Modifier(ModifierKey::LeftShift),
    RShift => Modifier(ModifierKey::RightShift),
    LAlt => Modifier(ModifierKey::LeftAlt),
    RAlt => Modifier(ModifierKey::RightAlt),
    LMeta => Modifier(ModifierKey::Super),
    RMeta => Modifier(ModifierKey::Super),
    Enter => System(SystemKey::Enter),
    Up => Arrow(ArrowKey::Up),
    Down => Arrow(ArrowKey::Down),
    Left => Arrow(ArrowKey::Left),
    Right => Arrow(ArrowKey::Right),
    Backspace => System(SystemKey::Backspace),
    CapsLock => Modifier(ModifierKey::Caps),
    Tab => System(SystemKey::Tab),
    _ => return None,
  })
}

impl PlatformControllerManager for Controller {
  type Ops = WindowsOps;

  fn try_create() -> Result<Self, Box<dyn std::error::Error>> {
    Ok(Self {
      virtual_device: WindowsOps::create_virtual_controller().unwrap(),
      left_stick: Arc::new(Mutex::new(JoyStickState::default())),
      right_stick: Arc::new(Mutex::new(JoyStickState::default())),
    })
  }
}
