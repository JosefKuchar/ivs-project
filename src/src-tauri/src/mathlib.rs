use dec::*;

// Number of digit precision
//TODO: Determine this properly
const PRECISION: usize = 200;

// Typedef for simpler usage, also should be used outside mode
pub type Dec = Decimal<PRECISION>;

pub fn add(a: Dec, b: Dec) -> Dec {
  unimplemented!()
}

pub fn subtract(a: Dec, b: Dec) -> Dec {
  unimplemented!()
}

pub fn multiply(a: Dec, b: Dec) -> Dec {
  unimplemented!()
}

pub fn divide(a: Dec, b: Dec) -> Result<Dec, String> {
  unimplemented!()
}

pub fn pow(a: Dec, b: Dec) -> Result<Dec, String> {
  unimplemented!()
}

pub fn root(a: Dec, b: Dec) -> Result<Dec, String> {
  unimplemented!()
}

pub fn factorial(a: Dec) -> Result<Dec, String> {
  unimplemented!()
}

pub fn abs(a: Dec) -> Dec {
  unimplemented!()
}

#[cfg(test)]
mod tests {
  use dec::*;

  #[test]
  fn add() {
    assert_eq!(
      super::add(Decimal::from(0.1), Decimal::from(0.2)),
      Decimal::from(0.3)
    );
  }
}
