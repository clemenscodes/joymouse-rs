mod error;

pub use error::PolarityError;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Polarity {
  Positive(f64),
  Negative(f64),
}

impl Polarity {
  pub fn magnitude(&self) -> f64 {
    match self {
      Polarity::Positive(val) | Polarity::Negative(val) => val.abs(),
    }
  }

  pub fn sign(&self) -> f64 {
    match self {
      Polarity::Positive(_) => 1.0,
      Polarity::Negative(_) => -1.0,
    }
  }
}

impl From<Polarity> for f64 {
  fn from(value: Polarity) -> Self {
    match value {
      Polarity::Positive(strength) => strength,
      Polarity::Negative(strength) => strength,
    }
  }
}

impl TryFrom<f64> for Polarity {
  type Error = PolarityError;

  fn try_from(value: f64) -> Result<Self, Self::Error> {
    if value > 0.0 {
      Ok(Self::Positive(value))
    } else if value < 0.0 {
      Ok(Self::Negative(value))
    } else {
      Err(PolarityError::InvalidPolarity(value))
    }
  }
}
