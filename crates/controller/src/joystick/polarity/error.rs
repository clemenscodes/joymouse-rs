#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PolarityError {
  InvalidPolarity(f64),
}

impl std::fmt::Display for PolarityError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      PolarityError::InvalidPolarity(polarity) => {
        writeln!(f, "Invalid polarity: {}", polarity)
      }
    }
  }
}

impl std::error::Error for PolarityError {}
