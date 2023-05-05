use core::ops::{Mul, MulAssign};

use lazy_static::lazy_static;

use crate::{ibig::{log::Logarithm, IBig}, fbig::fbig::ZERO as Z};

use super::FBig;

lazy_static! {
  pub static ref ZERO : FBig = Z.to_owned();
}

fn mul(lhs : FBig, rhs : FBig) -> FBig {
  assert!(lhs.b == rhs.b);

  // let p = if lhs.p > rhs.p {lhs.p} else {rhs.p};

  let f1 = lhs.p as isize - lhs.e;
  let f2 = rhs.p as isize - rhs.e;
  let f = f1 + f2;

  let num = lhs.n * rhs.n;
  let prec = num.log10().to_f32() as usize;
  // let exp = prec as isize - f;
  let exp = lhs.e + rhs.e;

  dbg!(exp, f, prec);

  if num == ZERO.n {return ZERO.to_owned()};

  return FBig::new(num, lhs.b, exp, prec);
}

impl Mul<FBig> for FBig {
  type Output = FBig;

  fn mul(self, rhs: FBig) -> Self::Output {
    return mul(self, rhs);
  }
}


impl MulAssign<FBig> for FBig {
  fn mul_assign(&mut self, rhs: FBig) {
      *self = mul(self.clone(), rhs);
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
    
    let expected = to_fbig_vector(vec![Str("30.0636".to_string()), Str("0".to_string()), Float64(0.0), Int64(70122), Float64(-0.4), Str("8335450802422672859553559685048242436".to_string())]);

    let res = apply(|x, y| x * y, a, b);

    assert_eq!(expected, res);

  }
}