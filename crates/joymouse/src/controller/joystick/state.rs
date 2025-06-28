use crate::controller::settings::SETTINGS;

#[derive(Default, Debug)]
pub struct JoyStickState {
  x: i32,
  y: i32,
}

impl JoyStickState {
  pub fn x(&mut self, x: i32) -> i32 {
    self.x += x * SETTINGS.sensitivity();
    self.x
  }

  pub fn y(&mut self, y: i32) -> i32 {
    self.y += y * SETTINGS.sensitivity();
    self.y
  }
}
