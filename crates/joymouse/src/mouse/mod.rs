use evdev::{AttributeSet, BusType, InputEvent, InputId, KeyCode, RelativeAxisCode, uinput::VirtualDevice};

#[derive(Debug)]
pub struct Mouse {
  virtual_device: VirtualDevice,
}

impl Mouse {
  pub fn try_create() -> Result<Self, Box<dyn std::error::Error>> {
    let builder = VirtualDevice::builder()?;

    let name = "JoyMouse Virtual Mouse";

    let vendor = 0x1234;
    let product = 0x5678;
    let version = 0x0100;
    let id = InputId::new(BusType::BUS_USB, vendor, product, version);

    let mut buttons = AttributeSet::<KeyCode>::new();
    buttons.insert(KeyCode::BTN_LEFT);
    buttons.insert(KeyCode::BTN_RIGHT);
    buttons.insert(KeyCode::BTN_MIDDLE);

    let mut axes = AttributeSet::<RelativeAxisCode>::new();
    axes.insert(RelativeAxisCode::REL_X);
    axes.insert(RelativeAxisCode::REL_Y);
    axes.insert(RelativeAxisCode::REL_WHEEL);

    let virtual_device = builder.name(&name).input_id(id).with_keys(&buttons)?.with_relative_axes(&axes)?.build()?;

    Ok(Self {
      virtual_device,
    })
  }

  pub fn emit(&mut self, event: InputEvent) {
    let events = vec![event];
    self.virtual_device.emit(&events).unwrap();
  }
}
