mod device;

use crate::windows::device::VirtualDevice;

use bindings::{JOYSTICK_KEYS, KEYBOARD_BUTTON_MAP};
use controller::{
  Axis, ButtonEvent, ControllerError, ControllerEvent, ControllerEventEmitter, JoyStick,
  JoyStickEvent, JoyStickState, PlatformControllerManager, PlatformControllerOps, Polarity, State,
  VirtualController, VirtualControllerCore,
};
use io::{AlphabeticKey, ArrowKey, FunctionKey, Key, ModifierKey, MouseKey, NumericKey, SystemKey};

use std::{
  sync::{Arc, Mutex},
  time::Duration,
};

use device_query::{DeviceEvents, DeviceEventsHandler, Keycode, MouseButton};

use windows::{
  core::PCWSTR,
  Win32::{
    Devices::HumanInterfaceDevice::{HID_USAGE_GENERIC_MOUSE, HID_USAGE_PAGE_GENERIC},
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
      Input::{
        GetRawInputData, RegisterRawInputDevices, HRAWINPUT, MOUSE_MOVE_ABSOLUTE, RAWINPUT,
        RAWINPUTDEVICE, RAWINPUTHEADER, RIDEV_INPUTSINK, RID_INPUT, RIM_TYPEMOUSE,
      },
      WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, RegisterClassW,
        TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, HWND_MESSAGE, MSG, WM_INPUT,
        WNDCLASSW, WS_OVERLAPPEDWINDOW,
      },
    },
  },
};

pub struct Controller {
  virtual_device: <WindowsOps as PlatformControllerOps>::VirtualDevice,
  left_stick: Arc<Mutex<JoyStickState>>,
  right_stick: Arc<Mutex<JoyStickState>>,
}

impl ControllerEventEmitter for Controller {
  fn emit(&mut self, events: &[ControllerEvent]) -> Result<(), ControllerError> {
    let left_stick = { self.left_stick.lock().unwrap().clone() };
    let right_stick = { self.right_stick.lock().unwrap().clone() };
    self.virtual_device.emit(events, &left_stick, &right_stick)
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
    use Axis::*;
    use JoyStick::*;
    use Polarity::*;
    use State::*;

    let handler = DeviceEventsHandler::new(Duration::from_millis(10))
      .expect("Failed to create DeviceEventsHandler");

    let _g_key_down_controller = Arc::clone(&controller);
    let _g_key_down = handler.on_key_down(move |key: &Keycode| {
      let mut controller = _g_key_down_controller.lock().unwrap();
      let state = Pressed;
      if let Some(key) = map_key(key) {
        if JOYSTICK_KEYS.key_is_joystick_key(key) {
          let axis_polarity = match key {
            k if JOYSTICK_KEYS.key_is_forward(k) => Some((Y, Positive(1))),
            k if JOYSTICK_KEYS.key_is_backward(k) => Some((Y, Negative(1))),
            k if JOYSTICK_KEYS.key_is_port(k) => Some((X, Negative(1))),
            k if JOYSTICK_KEYS.key_is_starboard(k) => Some((X, Positive(1))),
            _ => None,
          };

          if let Some((axis, polarity)) = axis_polarity {
            let event = ControllerEvent::from(JoyStickEvent::new(Left, axis, polarity, state));
            controller.handle_event(event).unwrap();
          }
        } else if let Some(&button) = KEYBOARD_BUTTON_MAP.get(&key) {
          let event = ControllerEvent::from(ButtonEvent::new(button, state));
          controller.handle_event(event).unwrap();
        }
      }
    });

    let _g_key_up_controller = Arc::clone(&controller);
    let _g_key_up = handler.on_key_up(move |key: &Keycode| {
      let mut controller = _g_key_up_controller.lock().unwrap();
      let state = Released;
      if let Some(key) = map_key(key) {
        if JOYSTICK_KEYS.key_is_joystick_key(key) {
          let axis_polarity = match key {
            k if JOYSTICK_KEYS.key_is_forward(k) => Some((Y, Positive(1))),
            k if JOYSTICK_KEYS.key_is_backward(k) => Some((Y, Negative(1))),
            k if JOYSTICK_KEYS.key_is_port(k) => Some((X, Negative(1))),
            k if JOYSTICK_KEYS.key_is_starboard(k) => Some((X, Positive(1))),
            _ => None,
          };

          if let Some((axis, polarity)) = axis_polarity {
            let event = ControllerEvent::from(JoyStickEvent::new(Left, axis, polarity, state));
            controller.handle_event(event).unwrap();
          }
        } else if let Some(button) = KEYBOARD_BUTTON_MAP.get(&key) {
          let button_event = ButtonEvent::new(*button, state);
          let controller_event = ControllerEvent::from(button_event);
          controller.handle_event(controller_event).unwrap();
        }
      }
    });

    let _g_mouse_down_controller = Arc::clone(&controller);
    let _g_mouse_down = handler.on_mouse_down(move |btn: &MouseButton| {
      let mut controller = _g_mouse_down_controller.lock().unwrap();
      let state = Pressed;
      if let Some(key) = map_mouse_button(btn) {
        if let Some(button) = KEYBOARD_BUTTON_MAP.get(&key) {
          let button_event = ButtonEvent::new(*button, state);
          let controller_event = ControllerEvent::from(button_event);
          controller.handle_event(controller_event).unwrap();
        }
      }
    });

    let _g_mouse_up_controller = Arc::clone(&controller);
    let _g_mouse_up = handler.on_mouse_up(move |btn: &MouseButton| {
      let mut controller = _g_mouse_up_controller.lock().unwrap();
      let state = Released;
      if let Some(key) = map_mouse_button(btn) {
        if let Some(button) = KEYBOARD_BUTTON_MAP.get(&key) {
          let button_event = ButtonEvent::new(*button, state);
          let controller_event = ControllerEvent::from(button_event);
          controller.handle_event(controller_event).unwrap();
        }
      }
    });

    unsafe {
      let class_name: Vec<u16> = "RawMouseWin".encode_utf16().chain([0]).collect();

      let hinstance = GetModuleHandleW(None).expect("GetModuleHandleW failed");

      let wc = WNDCLASSW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wndproc),
        hInstance: hinstance.into(),
        lpszClassName: PCWSTR(class_name.as_ptr()),
        ..Default::default()
      };

      RegisterClassW(&wc);

      let hwnd = CreateWindowExW(
        Default::default(),
        PCWSTR(class_name.as_ptr()),
        PCWSTR(class_name.as_ptr()),
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        Some(HWND_MESSAGE),
        None,
        Some(hinstance.into()),
        None,
      )
      .expect("CreateWindowExW failed");

      let rid = RAWINPUTDEVICE {
        usUsagePage: HID_USAGE_PAGE_GENERIC,
        usUsage: HID_USAGE_GENERIC_MOUSE,
        dwFlags: RIDEV_INPUTSINK,
        hwndTarget: hwnd,
      };

      RegisterRawInputDevices(&[rid], size_of::<RAWINPUTDEVICE>() as u32)
        .expect("RegisterRawInputDevices failed");

      let mut msg = MSG::default();
      loop {
        while GetMessageW(&mut msg, None, 0, 0).into() {
          let _ = TranslateMessage(&msg);
          DispatchMessageW(&msg);
        }
      }
    }
  }
}

fn map_mouse_button(button: &MouseButton) -> Option<Key> {
  use Key::*;
  use MouseKey::*;

  Some(match button {
    1 => Mouse(Left),
    2 => Mouse(Right),
    3 => Mouse(Middle),
    4 => Mouse(Side),
    5 => Mouse(Extra),
    _ => return None,
  })
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

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
  if msg == WM_INPUT {
    let hraw = HRAWINPUT(lparam.0 as _);
    let mut size: u32 = 0;
    let header_size = std::mem::size_of::<RAWINPUTHEADER>() as u32;
    GetRawInputData(hraw, RID_INPUT, None, &mut size, header_size);

    let mut buf = vec![0u8; size as usize];
    if GetRawInputData(hraw, RID_INPUT, Some(buf.as_mut_ptr() as *mut _), &mut size, header_size)
      != u32::MAX
    {
      let raw = &*(buf.as_ptr() as *const RAWINPUT);
      if raw.header.dwType == RIM_TYPEMOUSE.0 {
        let m = unsafe { raw.data.mouse };
        if (m.usFlags.0 & MOUSE_MOVE_ABSOLUTE.0) == 0 {
          println!("dx={} dy={}", m.lLastX, m.lLastY);
        }
      }
    }
  }
  DefWindowProcW(hwnd, msg, wparam, lparam)
}