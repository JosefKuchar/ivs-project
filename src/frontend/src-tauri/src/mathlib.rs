pub fn add(a: f64, b: f64) -> f64 {
  0.0
}

pub fn subtract(a: f64, b: f64) -> f64 {
  0.0
}

pub fn multiply(a: f64, b: f64) -> f64 {
  0.0
}

pub fn divide(a: f64, b: f64) -> f64 {
  0.0
}

mod tests {
    use super::*;
    use float_cmp::*;

    #[test]
    fn test_add() {
        assert!(approx_eq!(f64, add(0.1, 0.2), 0.3));
    }
}
