use crate::controller::settings::SETTINGS;

#[derive(Default, Debug)]
pub struct JoyStickState {
  x: i32,
  y: i32,
}

impl JoyStickState {
  pub const MAX: i32 = 32767;
  pub const MIN: i32 = -32768;

  pub fn x(&mut self, x: i32) -> i32 {
    let sensitivy = SETTINGS.sensitivity();

    let delta = x * sensitivy;

    if self.x + delta >= Self::MAX {
      self.x = Self::MAX;
      return self.x;
    }

    if self.x + delta <= Self::MIN {
      self.x = Self::MIN;
      return self.x;
    }

    self.x += delta;
    self.x
  }

  pub fn y(&mut self, y: i32) -> i32 {
    let sensitivy = SETTINGS.sensitivity();

    let delta = y * sensitivy;

    if self.y + delta >= Self::MAX {
      self.y = Self::MAX;
      return self.y;
    }

    if self.y + delta <= Self::MIN {
      self.y = Self::MIN;
      return self.y;
    }

    self.y += delta;
    self.y
  }
}
