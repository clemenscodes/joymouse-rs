#[derive(Debug)]
pub enum State {
  Pressed,
  Released,
  Held,
}

impl State {
  pub fn as_value(&self) -> i32 {
    match self {
      State::Released => 0,
      State::Pressed => 1,
      State::Held => 2,
    }
  }
}
