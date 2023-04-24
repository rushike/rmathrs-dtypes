use lazy_static::lazy_static;

use crate::ibig::IBig;
lazy_static! {
  static ref TEN : IBig = IBig::from(10);
}

pub trait DecimalShift {
  fn shld(&self, shift: usize) -> IBig;
  fn shrd(&self, shift: usize) -> IBig;
}

impl DecimalShift for IBig {
  /// shld operation will left shift `num` by given number of `shift` digits
  /// 
  /// # Examples
  /// 1. `shld(32, 2) => 3200`
  /// 
  fn shld(&self, shift:usize) -> IBig {
    TEN.pow(shift) * self 
  }
  /// shrd operation will right shift `num` by given number of `shift` digits
  /// 
  /// # Examples
  /// 1. `shrd(3200, 2) => 32`
  /// 
  fn shrd(&self, shift:usize) -> IBig {
    self / TEN.pow(shift)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_shld() { 
    let num = IBig::from(32);
    assert_eq!(num.shld(2), IBig::from(3200));
  }

  #[test]
  fn test_shrd() {
    let num = IBig::from(123000);
    assert_eq!(num.shrd(3), IBig::from(123));
  }
}
