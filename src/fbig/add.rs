use core::ops::{Add, AddAssign, Sub, SubAssign};
use std::cmp::max;

use super::FBig;

use crate::ibig::{shift::decimal::DecimalShift, IBig, log::Logarithm, ops::Abs};

fn add_or_sub(lhs: &FBig, rhs: &FBig, op : fn(&IBig, &IBig) -> IBig) -> FBig {
  assert!(lhs.b == rhs.b);

  let f1 = lhs.p as isize - lhs.e;
  let f2 = rhs.p as isize - rhs.e;
  let f = max(f1, f2);

  let num = if f1 > f2 { 
    // If fractional part of left is > than fraction part of right
    op(&lhs.n, &rhs.n.shld((f1 - f2) as usize))
  } else  { 
    op(&lhs.n.shld((f2 - f1) as usize), &rhs.n) 
  };
  
  let prec = num.log10().to_f32() as usize;
  let exp = prec as isize - f;

  return FBig::new(num, lhs.b, exp, prec);
}

impl Add<FBig> for FBig {
  type Output = FBig;

  fn add(self, rhs: Self) -> Self {
      return add_or_sub(&self, &rhs, 
        |a, b| a + b);
  }
}

impl Sub<FBig> for FBig {
  type Output = FBig; 

  fn sub(self, rhs: Self) -> Self {
      return add_or_sub(&self, &rhs, 
        |a, b| a - b);
  }
}

impl AddAssign<FBig> for FBig {
    fn add_assign(&mut self, rhs: FBig) {
        *self = add_or_sub(&self, &rhs, 
          |a, b| a + b);
    }
}

impl SubAssign<FBig> for FBig {
  fn sub_assign(&mut self, rhs: FBig) {
      *self = add_or_sub(&self, &rhs, 
        |a, b| a - b);
  }
}

#[cfg(test)]
mod tests{
  use crate::fbig::testutils::{to_fbig_vector, apply};

  use super::FBig;

  use super::super::testutils::TestInput::{Uint16, Int64, Float64, Str};

  #[test]
  fn test_add() {
    let a = to_fbig_vector(vec![Str(String::from("100.212")),Int64(0), Float64(89.898), Int64(78), Uint16(1), Str("123567876876876345435435424534".to_string())]);

    let b = to_fbig_vector(vec![Str("0.3".to_string()), Str("121.90".to_string()), Str("0".to_string()), Str(String::from("899")), Float64(-0.4), Int64(67456454)]);
    
    let expected = to_fbig_vector(vec![Str("100.512".to_string()), Str("121.90".to_string()), Float64(89.898), Int64(977), Float64(0.6), Str("123567876876876345435502880988".to_string())]);

    let res = apply(|x, y| x + y, a, b);

    assert_eq!(expected, res);
  }
  
  #[test]
  fn test_sub() {
    let a = to_fbig_vector(vec![Str(String::from("100.212")),Int64(0), Float64(89.898), Int64(78), Uint16(1), Str("123567876876876345435435424534".to_string())]);

    let b = to_fbig_vector(vec![Str("0.3".to_string()), Str("121.90".to_string()), Str("0".to_string()), Str(String::from("899")), Float64(-0.4), Int64(67456454)]);
    
    let expected = to_fbig_vector(vec![Str("99.912".to_string()), Str("-121.90".to_string()), Float64(89.898), Int64(-821), Float64(1.4), Str("123567876876876345435367968080".to_string())]);

    let res = apply(|x, y| x - y, a, b);

    assert_eq!(expected, res);
  }
}