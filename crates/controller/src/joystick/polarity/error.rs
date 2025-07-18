#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PolarityError {
  InvalidPolarity(i32),
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
