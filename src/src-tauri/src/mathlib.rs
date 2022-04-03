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
  if b.exponent() != 0 || b < Decimal::from(1) {
    Err("Only natural exponents are supported!".to_string())
  } else {
    let mut mut_a = a.clone();
    let mut mut_b = b.clone();
    ctx.pow(&mut mut_a, &mut mut_b);
    Ok(mut_a)
  }
}

pub fn root(a: Dec, b: Dec) -> Result<Dec, String> {
  let mut ctx = Context::<Dec>::default();
  if b.exponent() != 0 || b.is_negative() || a.is_negative() {
    Err("Only root of natural numbers is supported!".to_string())
  } else if a.is_negative() && b % Dec::from(2) != Dec::from(1) {
    Err("Only odd roots of negative values can be calculated!".to_string())
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
  fn add_1() {
    assert_eq!(
      super::add(Decimal::from(0.1), Decimal::from(0.2)),
      Decimal::from(0.3)
    );
  }

  #[test]
  fn add_2() {
    assert_eq!(
      super::add(Decimal::from(2.1), Decimal::from(2)),
      Decimal::from(4.1)
    );
  }

  #[test]
  fn add_3() {
    assert_eq!(
      super::add(Decimal::from(8453216), Decimal::from(5462132)),
      Decimal::from(13905348)
    );
  }

  #[test]
  fn add_4() {
    assert_eq!(
      super::add(Decimal::from(0), Decimal::from(0.2)),
      Decimal::from(0.2)
    );
  }

  #[test]
  fn add_5() {
    assert_eq!(
      super::add(Decimal::from(7), Decimal::from(-15)),
      Decimal::from(-8)
    );
  }

  #[test]
  fn subtract_1() {
    assert_eq!(
      super::subtract(Decimal::from(5), Decimal::from(10)),
      Decimal::from(-5)
    );
  }

  #[test]
  fn subtract_2() {
    assert_eq!(
      super::add(Decimal::from(2), Decimal::from(0.3)),
      Decimal::from(1.7)
    );
  }

  #[test]
  fn subtract_3() {
    assert_eq!(
      super::subtract(Decimal::from(0.7), Decimal::from(0.6)),
      Decimal::from(0.1)
    );
  }

  #[test]
  fn subtract_4() {
    assert_eq!(
      super::subtract(Decimal::from(0), Decimal::from(-23)),
      Decimal::from(23)
    );
  }

  #[test]
  fn subtract_5() {
    assert_eq!(
      super::subtract(Decimal::from(0), Decimal::from(69420)),
      Decimal::from(-69420)
    );
  }

  #[test]
  fn multiply_1() {
    assert_eq!(
      super::multiply(Decimal::from(652), Decimal::from(0)),
      Decimal::from(0)
    );
  }

  #[test]
  fn multiply_2() {
    assert_eq!(
      super::multiply(Decimal::from(7), Decimal::from(191)),
      Decimal::from(1337)
    );
  }

  #[test]
  fn multiply_3() {
    assert_eq!(
      super::multiply(Decimal::from(0), Decimal::from(0)),
      Decimal::from(0)
    );
  }

  #[test]
  fn multiply_4() {
    assert_eq!(
      super::multiply(Decimal::from(8), Decimal::from(-32)),
      Decimal::from(-256)
    );
  }

  #[test]
  fn multiply_5() {
    assert_eq!(
      super::multiply(Decimal::from(-7), Decimal::from(-6)),
      Decimal::from(42)
    );
  }

  #[test]
  fn multiply_6() {
    assert_eq!(
      super::multiply(Decimal::from(-0.1), Decimal::from(10)),
      Decimal::from(-1)
    );
  }

  #[test]
  fn multiply_7() {
    assert_eq!(
      super::multiply(Decimal::from(0.2), Decimal::from(0.5)),
      Decimal::from(0.1)
    );
  }

  #[test]
  fn divide_1() {
    assert_eq!(
      super::divide(Decimal::from(48), Decimal::from(0.0005)),
      Decimal::from(240000)
    );
  }

  #[test]
  fn divide_2() {
    assert_eq!(
      super::divide(Decimal::from(0), Decimal::from(6)),
      Decimal::from(0)
    );
  }

  #[test]
  fn divide_3() {
    assert!(super::divide(Decimal::from(-531), Decimal::from(0)).is_err());
  }

  #[test]
  fn divide_4() {
    assert_eq!(
      super::divide(Decimal::from(-531), Decimal::from(-15)),
      Decimal::from(35.4)
    );
  }
  #[test]
  fn pow_1() {
    assert_eq!(
      super::pow(Decimal::from(-5), Decimal::from(4)),
      Decimal::from(625)
    );
  }
  #[test]
  fn pow_2() {
    assert_eq!(
      super::pow(Decimal::from(-2.5), Decimal::from(3)),
      Decimal::from(-15.625)
    );
  }
  #[test]
  fn pow_3() {
    assert!(super::pow(Decimal::from(0.4), Decimal::from(0.1)).is_err());
  }
  #[test]
  fn pow_4() {
    assert!(super::pow(Decimal::from(0.4), Decimal::from(-1)).is_err());
  }
  #[test]
  fn pow_5() {
    assert!(super::pow(Decimal::from(0.4), Decimal::from(0)).is_err());
  }

  #[test]
  fn root_1() {
    assert_eq!(
      super::root(Decimal::from(-8), Decimal::from(3)),
      Decimal::from(-2)
    );
  }

  #[test]
  fn root_2() {
    assert_eq!(
      super::root(Decimal::from(0), Decimal::from(2)),
      Decimal::from(0)
    );
  }

  #[test]
  fn root_3() {
    assert!(super::root(Decimal::from(-4), Decimal::from(2)).is_err());
  }

  #[test]
  fn root_4() {
    assert!(super::root(Decimal::from(-64), Decimal::from(6)).is_err());
  }

  #[test]
  fn factorial_1() {
    assert!(super::factorial(Decimal::from(-1568)).is_err());
  }

  #[test]
  fn factorial_2() {
    assert_eq!(super::factorial(Decimal::from(0)), Decimal::from(1));
  }

  #[test]
  fn factorial_3() {
    assert_eq!(
      super::factorial(Decimal::from(15)),
      Decimal::from(1307674368000)
    );
  }

  #[test]
  fn factorial_4() {
    assert!(super::factorial(Decimal::from(0.1)).is_err());
  }

  #[test]
  fn factorial_5() {
    assert_eq!(super::factorial(Decimal::from(1)), Decimal::from(1));
  }

  #[test]
  fn factorial_6() {
    assert_eq!(super::factorial(Decimal::from(9)), Decimal::from(362880));
  }

  #[test]
  fn abs_1() {
    assert_eq!(super::abs(Decimal::from(-14556)), Decimal::from(14556));
  }

  #[test]
  fn abs_2() {
    assert_eq!(super::abs(Decimal::from(856)), Decimal::from(856));
  }

  #[test]
  fn abs_3() {
    assert_eq!(super::abs(Decimal::from(-0.8569)), Decimal::from(0.8569));
  }

  #[test]
  fn abs_4() {
    assert_eq!(super::abs(Decimal::from(0)), Decimal::from(0));
  }
}
