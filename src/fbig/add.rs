use core::ops::{Add, AddAssign, Sub, SubAssign};
use std::cmp::max;

use super::FBig;

use crate::ibig::{shift::decimal::DecimalShift, IBig};

fn add_or_sub(lhs: &FBig, rhs: &FBig, op : fn(&IBig, &IBig) -> IBig) -> FBig {
  assert!(lhs.b == rhs.b);

  let emax = max(lhs.e, rhs.e);
  let pmax = max(lhs.p, rhs.p);
  if lhs.e > lhs.p as isize  && rhs.e >= rhs.p as isize {
    return FBig::new(&lhs.n + &rhs.n, lhs.b, emax, pmax);
  }

  let f1 = lhs.p as isize - lhs.e;
  let f2 = rhs.p as isize - rhs.e;
  let f = max(f1, f2);

  let num = if f1 > f2 { 
    // If fractional part of left is > than fraction part of right
    op(&lhs.n, &rhs.n.shld((f1 - f2) as usize))
  } else  { 
    op(&lhs.n.shld((f2 - f1) as usize), &rhs.n) 
  };
  
  // dbg!(lhs.p, rhs.p, pmax);
  return FBig::new(num, lhs.b, emax, pmax);
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
  use crate::fbig::testutils::{to_fbig_vec, apply_operation};

use super::FBig;

  use super::super::testutils::TestInput::{Uint16, Int64, Float64, Str, self};

  #[test]
  fn test_single() {
    let a = FBig::from(1.0);
    let b = FBig::from(-0.4);
    println!("a, b = {:?}, {:?}", a, b);
    println!("a - b : {:?}", a - b);
  }
  
  #[test]
  fn test_sub() {
    let a = to_fbig_vec(vec![TestInput::Str(String::from("100.212")),Int64(0), Float64(89.898), Int64(78), Uint16(1)]);
    dbg!(&a);
    let b = to_fbig_vec(vec![Str("0.3".to_string()), Str("121.90".to_string()), Str("0".to_string()), Str(String::from("899")), Float64(-0.4)]);
    dbg!(&b);
    let expected = to_fbig_vec(vec![Str("99.912".to_string()), Str("-121.90".to_string()), Float64(89.898), Int64(-821), Float64(1.4)]);
   
    let res = apply_operation(a, b);
    dbg!(&res);
    assert_eq!(expected, res);
    
  }
}