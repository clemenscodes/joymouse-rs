use crate::controller::settings::SETTINGS;

#[derive(Debug, Default)]
pub struct JoyStickState {
  pub x: i32,
  pub y: i32,
}

impl JoyStickState {
  pub const MAX: i32 = 32767;

  pub fn tilt(&mut self, dx: i32, dy: i32) -> i32 {
    let sensitivity = SETTINGS.sensitivity();
    self.x += dx * sensitivity;
    self.y += dy * sensitivity;

    let magnitude = ((self.x as f64).powi(2) + (self.y as f64).powi(2)).sqrt();
    if magnitude > Self::MAX as f64 {
      let scale = Self::MAX as f64 / magnitude;
      self.x = (self.x as f64 * scale).round() as i32;
      self.y = (self.y as f64 * scale).round() as i32;
    }

    if dx != 0 {
      self.x
    } else {
      self.y
    }
  }

  pub fn recenter(&mut self) {
    self.x = 0;
    self.y = 0;
  }
}
