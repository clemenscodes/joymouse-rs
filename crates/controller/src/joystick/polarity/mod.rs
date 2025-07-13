mod error;

pub use error::PolarityError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Polarity {
  Positive(i32),
  Negative(i32),
}

impl Polarity {
  pub fn magnitude(&self) -> i32 {
    match self {
      Polarity::Positive(val) | Polarity::Negative(val) => val.abs(),
    }
  }

  pub fn sign(&self) -> i32 {
    match self {
      Polarity::Positive(_) => 1,
      Polarity::Negative(_) => -1,
    }
  }
}

impl From<Polarity> for i32 {
  fn from(value: Polarity) -> Self {
    match value {
      Polarity::Positive(strength) => strength,
      Polarity::Negative(strength) => strength,
    }
  }
}
