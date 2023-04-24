use core::ops::{Mul, MulAssign};

use super::FBig;

fn mul(lhs : FBig, rhs : FBig) -> FBig {
  assert!(lhs.b == rhs.b);

  let p = if lhs.p > rhs.p {lhs.p} else {rhs.p};

  let f1 = lhs.e - lhs.p as isize; 
  let f2 = rhs.e - rhs.p as isize;
  let f = f1 + f2;

  return FBig::new(lhs.n * rhs.n, lhs.b, f, p );
}

impl Mul<FBig> for FBig {
  type Output = FBig;

  fn mul(self, rhs: FBig) -> Self::Output {
    return mul(self, rhs);
  }
}

#[cfg(test)]
mod tests {
    use crate::fbig::testutils::{to_fbig_vector, apply};

    use super::super::testutils::TestInput::{Uint16, Int64, Float64, Str};

  #[test]
  fn test_mul() {
    let a = to_fbig_vector(vec![Str(String::from("100.212")),Int64(0), Float64(89.898), Int64(78), Uint16(1), Str("123567876876876345435435424534".to_string())]);

    let b = to_fbig_vector(vec![Str("0.3".to_string()), Str("121.90".to_string()), Str("0".to_string()), Str(String::from("899")), Float64(-0.4), Int64(67456454)]);
    
    let expected = to_fbig_vector(vec![Str("100.512".to_string()), Str("121.90".to_string()), Float64(89.898), Int64(977), Float64(0.6), Str("123567876876876345435502880988".to_string())]);

    let res = apply(|x, y| x + y, a, b);

    assert_eq!(expected, res);

  }
}