use dec::*;

// Number of digit precision
//TODO: Determine this properly
const PRECISION: usize = 200;

// Typedef for simpler usage, also should be used outside mode
pub type Dec = Decimal<PRECISION>;

pub fn add(a: Dec, b: Dec) -> Dec {
  a + b
}

pub fn subtract(a: Dec, b: Dec) -> Dec {
  a - b
}

pub fn multiply(a: Dec, b: Dec) -> Dec {
  a * b
}

pub fn divide(a: Dec, b: Dec) -> Result<Dec, String> {
  if b.is_zero() {
    Err("Cannot divide by 0!".to_string())
  } else {
    Ok(a / b)
  }
}

pub fn pow(a: Dec, b: Dec) -> Result<Dec, String> {
  let mut ctx = Context::<Dec>::default();
  if b.exponent() != 0 {
    Err("Only natural exponents are supported!".to_string())
  } else {
    let mut mut_a = a.clone();
    let mut mut_b = b.clone();
    ctx.pow(&mut mut_a, &mut mut_b);
    Ok(mut_a)
  }
}

pub fn root(a: Dec) -> Result<Dec, String> {
  let mut ctx = Context::<Dec>::default();
  if a.is_negative() {
    Err("Invalid base of square root!".to_string())
  } else {
    let mut mut_a = a.clone();
    ctx.sqrt(&mut mut_a);
    Ok(mut_a)
  }
}

pub fn factorial(a: Dec) -> Result<Dec, String> {
  let mut ctx = Context::<Dec>::default();
  if a.exponent() != 0 || a.is_negative() {
    Err("Cannot calculate factorial!".to_string())
  } else {
    let b = ctx.try_into_u128(a).unwrap();
    Ok(ctx.from_u128((1..=b).product()))
  }
}

pub fn abs(a: Dec) -> Dec {
  let mut ctx = Context::<Dec>::default();
  let mut mut_a = a.clone();
  ctx.abs(&mut mut_a);
  mut_a
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
