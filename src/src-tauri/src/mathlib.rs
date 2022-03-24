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
      Decimal::from(0.3);
    );
  }

  #[test]
  fn add() {
    assert_eq! (
        super::add(Decimal::from(7), Decimal::from(-5)),
        Decimal::from(2);
    );
  }
  
  #[test] // potreba pojmout celle R jako mozny vysledek
  fn substract() {
      assert_eq! (
          super::substract(Decimal::from(0), Decimal::from(0)),
          Decimal::from();
      );
  }

  #[test]
  fn substract() {
      assert_eq! (
          super::substract(Decimal::from(0), Decimal::((-1) * from(0))),
          Decimal::from(0);
      );
  }


  #[test]
  fn multiply() {
      assert_eq! (
          super::multiply(Decimal::from(0), Decimal::((-1) * from(0))),
          Decimal::((-1) * from(0));
      );
  }

  #[test]
  fn multiply() {
      assert_eq! (
          super::multiply(Decimal::from(0), Decimal::from(0)),
          Decimal::from(0);
      );
  }

  #[test]
  fn multiply() {
      assert_eq! (
          super::multiply(Decimal::((-1) * from(0)), Decimal::((-1) * from(0))),
          Decimal::from(0);
      );
  }

  #[test]
  fn divide() {
      assert_eq! (
          super::divide(Decimal::((-1) * from(0))), Decimal::from(0.000000000000000000000000001)),
          Decimal::((-1) * from(0)));
      );
  }

  #[test]
  fn divide() {
      assert_eq! (
          super::divide(Decimal::from(0), Decimal::from(0.000000000000000000000000001)),
          Decimal::from(0);
      );
  }

  #[test]
  fn divide() {
      should_panic! (
          super::divide(Decimal::from(0), Decimal::(0)),
          Decimal::from(0));
      );
  }

  #[test]  // potreba pojmout celle R jako mozny vysledek
  fn pow() {
      assert_eq! (
          super::pow(Decimal::((-1) * from(0)), Decimal::from(-48951)),
          Decimal::from();
      );
  }
  
  #[test]
  fn pow() {
      assert_eq! (
          super::pow(Decimal::from(0), Decimal::from(-48951)),
          Decimal::from(0));
      );
  }

  

  #[test]
  fn root() {
      should_panic! (
          super::root(Decimal::from(-18884), Decimal::from(2)),
          Decimal::from(0));
      );
  }

  #[test]
  fn root() {
      assert_eq! (
          super::root(Decimal::from(0), Decimal::from(1)),
          Decimal::from(0));
      );
  }

  #[test]
  fn factorial() {
      should_panic! (
          super::factorial(Decimal::from(-1568)),
          Decimal::from(1);
      );
  }

  #[test]
  fn factorial() {
      assert_eq! (
          super::factorial(Decimal::from(0) ),
          Decimal::from(1);
      );
  }

  #[test]
  fn abs() {
      assert_eq! (
          super::abs(Decimal::from(-14556)),
          Decimal::((-1)*from(-14556)));
      );
  }
}