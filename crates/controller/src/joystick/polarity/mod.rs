mod error;

pub use error::PolarityError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Polarity {
  Positive(i32),
  Negative(i32),
  Neutral,
}

impl Polarity {
  pub fn magnitude(&self) -> i32 {
    match self {
      Polarity::Positive(val) | Polarity::Negative(val) => val.abs(),
      Polarity::Neutral => 0,
    }
  }

  pub fn sign(&self) -> i32 {
    match self {
      Polarity::Positive(_) => 1,
      Polarity::Negative(_) => -1,
      Polarity::Neutral => 0,
    }
  }
}

impl From<Polarity> for i32 {
  fn from(value: Polarity) -> Self {
    match value {
      Polarity::Positive(strength) => strength,
      Polarity::Negative(strength) => strength,
      Polarity::Neutral => 0,
    }
  }
}

impl From<Polarity> for f64 {
  fn from(value: Polarity) -> Self {
    match value {
      Polarity::Positive(strength) => strength as f64,
      Polarity::Negative(strength) => strength as f64,
      Polarity::Neutral => 0.0,
    }
  }
}

impl From<i32> for Polarity {
  fn from(value: i32) -> Self {
    if value > 0 {
      Self::Positive(value)
    } else if value < 0 {
      Self::Negative(value)
    } else {
      Self::Neutral
    }
  }
}

impl From<f64> for Polarity {
  fn from(value: f64) -> Self {
    if value > 0.0 {
      Self::Positive(value as i32)
    } else if value < 0.0 {
      Self::Negative(value as i32)
    } else {
      Self::Neutral
    }
  }
}
