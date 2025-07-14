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

impl From<Polarity> for f64 {
  fn from(value: Polarity) -> Self {
    match value {
      Polarity::Positive(strength) => strength as f64,
      Polarity::Negative(strength) => strength as f64,
    }
  }
}

impl TryFrom<i32> for Polarity {
  type Error = PolarityError;

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    if value > 0 {
      Ok(Self::Positive(value))
    } else if value < 0 {
      Ok(Self::Negative(value))
    } else {
      Err(PolarityError::InvalidPolarity(value))
    }
  }
}

impl TryFrom<f64> for Polarity {
  type Error = PolarityError;

  fn try_from(value: f64) -> Result<Self, Self::Error> {
    if value > 0.0 {
      Ok(Self::Positive(value as i32))
    } else if value < 0.0 {
      Ok(Self::Negative(value as i32))
    } else {
      Err(PolarityError::InvalidPolarity(value as i32))
    }
  }
}
