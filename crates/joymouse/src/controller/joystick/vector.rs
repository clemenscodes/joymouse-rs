#[derive(Default, Debug, Copy, Clone)]
pub struct Vector {
  dx: i32,
  dy: i32,
}

impl Vector {
  pub fn new(dx: i32, dy: i32) -> Self {
    Self {
      dx,
      dy,
    }
  }

  pub fn dx(&self) -> i32 {
    self.dx
  }

  pub fn dy(&self) -> i32 {
    self.dy
  }
}
