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

pub fn divide(a: Dec, b: Dec) -> Option<Dec> {
  unimplemented!()
}

pub fn pow(a: Dec, b: Dec) -> Option<Dec> {
  unimplemented!()
}

pub fn root(a: Dec, b: Dec) -> Option<Dec> {
  unimplemented!()
}

pub fn factorial(a: Dec) -> Option<Dec> {
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

  #[test]
  fn add() {
    assert_eq!(
      super::add(Decimal::from(2.1), Decimal::from(2)),
      Decimal::from(4.1)
    );
  }

  #[test]
  fn add() {
    assert_eq!(
      super::add(Decimal::from(8453216), Decimal::from(5462132)),
      Decimal::from(13905348)
    );
  }

  #[test]
  fn add() {
    assert_eq!(
      super::add(Decimal::from(0), Decimal::from(0.2)),
      Decimal::from(0.2)
    );
  }

  #[test]
  fn add() {
    assert_eq!(
      super::add(Decimal::from(7), Decimal::from(-15)),
      Decimal::from(-8)
    );
  }

  #[test]
  fn subtract() {
    assert_eq!(
      super::subtract(Decimal::from(5), Decimal::from(10)),
      Decimal::from(-5)
    );
  }

  #[test]
  fn subtract() {
    assert_eq!(
      super::add(Decimal::from(2), Decimal::from(0.3)),
      Decimal::from(1.7)
    );
  }

  #[test]
  fn subtract() {
    assert_eq!(
      super::subtract(Decimal::from(0.7), Decimal::from(0.6)),
      Decimal::from(0.1)
    );
  }

  #[test]
  fn subtract() {
    assert_eq!(
      super::subtract(Decimal::from(0), Decimal::from(-23)),
      Decimal::from(23)
    );
  }

  #[test]
  fn subtract() {
    assert_eq!(
      super::subtract(Decimal::from(0), Decimal::from(69420)),
      Decimal::from(-69420)
    );
  }

  #[test]
  fn multiply() {
    assert_eq!(
      super::multiply(Decimal::from(652), Decimal::from(0)),
      Decimal::from(0)
    );
  }

  #[test]
  fn multiply() {
    assert_eq!(
      super::multiply(Decimal::from(7), Decimal::from(191)),
      Decimal::from(1337)
    );
  }

  #[test]
  fn multiply() {
    assert_eq!(
      super::multiply(Decimal::from(), Decimal::from(0)),
      Decimal::from(0)
    );
  }

  #[test]
  fn multiply() {
    assert_eq!(
      super::multiply(Decimal::from(8), Decimal::from(-32)),
      Decimal::from(-256)
    );
  }

  #[test]
  fn multiply() {
    assert_eq!(
      super::multiply(Decimal::from(-7), Decimal::from(-6)),
      Decimal::from(42)
    );
  }

  #[test]
  fn divide() {
    assert_eq!(
      super::divide(Decimal::from(48), Decimal::from(0.0005)),
      Decimal::from(240000)
    );
  }

  #[test]
  fn divide() {
    assert_eq!(
      super::divide(Decimal::from(0), Decimal::from(6)),
      Decimal::from(0)
    );
  }

  #[test]
  #[should_panic]
  fn divide() {
    assert_eq!(
      super::divide(Decimal::from(-531), Decimal::from(0)),
      Decimal::from(0)
    );
  }

  #[test]
  fn divide() {
    assert_eq!(
      super::divide(Decimal::from(-531), Decimal::from(-15)),
      Decimal::from(35.4)
    );
  }

  #[test]
  fn pow() {
    assert_eq!(
      super::pow(Decimal::from(3), Decimal::from(-5)),
      Decimal::from(0.00411522633744855967078189300412)
    );
  }

  #[test]
  fn pow() {
    assert_eq!(
      super::pow(Decimal::from(0), Decimal::from(-48951)),
      Decimal::from(0)
    );
  }
  #[test]
  fn pow() {
    assert_eq!(
      super::pow(Decimal::from(2), Decimal::from(-4)),
      Decimal::from(0.0625)
    );
  }
  #[test]
  fn pow() {
    assert_eq!(
      super::pow(Decimal::from(-5), Decimal::from(4)),
      Decimal::from(625)
    );
  }
  #[test]
  fn pow() {
    assert_eq!(
      super::pow(Decimal::from(-3), Decimal::from(3)),
      Decimal::from(-27)
    );
  }

  #[test]
  fn root() {
    assert_eq!(
      super::root(Decimal::from(-8), Decimal::from(3)),
      Decimal::from(-2)
    );
  }

  #[test]
  fn root() {
    assert_eq!(
      super::root(Decimal::from(0), Decimal::from(2)),
      Decimal::from(0)
    );
  }

  #[test]
  #[should_panic]
  fn root() {
    assert_eq!(
      super::root(Decimal::from0(-4), Decimal::from(2)),
      Decimal::from(2)
    );
  }

  #[test]
  #[should_panic]
  fn root() {
    assert_eq!(
      super::root(Decimal::from0(-64), Decimal::from(6)),
      Decimal::from(2)
    );
  }

  #[test]
  #[should_panic]
  fn factorial() {
    assert_eq!(super::factorial(Decimal::from(-1568)), Decimal::from(1));
  }

  #[test]
  fn factorial() {
    assert_eq!(super::factorial(Decimal::from(0)), Decimal::from(1));
  }

  #[test]
  fn factorial() {
    assert_eq!(
      super::factorial(Decimal::from(15)),
      Decimal::from(1, 307, 674, 368, 000)
    );
  }

  #[test]
  fn factorial() {
    assert_eq!(super::factorial(Decimal::from(9)), Decimal::from(362, 880));
  }

  #[test]
  fn abs() {
    assert_eq!(super::abs(Decimal::from(-14556)), Decimal::from(14556));
  }

  #[test]
  fn abs() {
    assert_eq!(super::abs(Decimal::from(856)), Decimal::from(856));
  }

  #[test]
  fn abs() {
    assert_eq!(super::abs(Decimal::from(-0.8569)), Decimal::from(0.8569));
  }
}
