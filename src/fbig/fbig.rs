use lazy_static::lazy_static;

use std::str::FromStr;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{ibig::IBig, common::arch::word::{FloatWord, SignedWord, Word}};

/// Real 'd' represented as :
///    d = n * b ^ -e
/// 
/// e.g. 
///  0.323928398 = 0.323928398 * 10 ^ 0 = 0.3239283980 * 10 ^ 0
///      here n = 323928398, b = 10, e = 0, p = 9
///      here n = 3239283980, b = 10, e = 0, p =10
///  787600000 = 0.7876 * 10 ^ 9 = 0.78760 * 10 ^ 8
///      here n = 7876, b = 10, e = 9, p = 4
///      here n = 78760, b = 10, e = 9, p = 5
#[wasm_bindgen]
#[derive(Clone, Eq, Hash, Debug)]
pub struct FBig {
  pub(crate) n : IBig,
  pub(crate) b : usize,
  pub(crate) e : isize,
  pub(crate) p : usize
}

lazy_static! {
  static ref ZERO : FBig = FBig {
    n : IBig::from(0),
    b : 10,
    e : 0,
    p : 0
  };
}

#[wasm_bindgen]
impl FBig {
    pub fn new(n : IBig, b : usize, e : isize, p : usize) -> FBig {
      return FBig {
        n, b, e, p
      };
    }
}

macro_rules! from_impl_for_ints_to_fbig {
  ($t:ty) => {
    impl From<$t> for FBig {
        #[inline]
        fn from(num: $t) -> FBig {
          if num == 0 {return ZERO.to_owned()}
          let e  = (num as FloatWord).abs().log10().floor() as isize + 1;
          // println!("inp : {num}, op : {e}");
          return Self{
              n: IBig::from(num),
              b: 10,
              e,
              p: e as usize
          };
        }
    }
  }
}

from_impl_for_ints_to_fbig!(u8);
from_impl_for_ints_to_fbig!(u16);
from_impl_for_ints_to_fbig!(u32);
from_impl_for_ints_to_fbig!(u64);
from_impl_for_ints_to_fbig!(u128);
from_impl_for_ints_to_fbig!(i8);
from_impl_for_ints_to_fbig!(i16);
from_impl_for_ints_to_fbig!(i32);
from_impl_for_ints_to_fbig!(i64);
from_impl_for_ints_to_fbig!(i128);



impl From<f64> for FBig {
    fn from(num: f64) -> Self {
        let e  = num.abs().log10().floor() as isize + 1;
        let intstr = num.to_string().replace(".", "");
        let int : i64 = intstr.parse().unwrap();
        // println!("num : {} e : {e}, int : {int}, intstr : {intstr}", num.abs().log10().floor());
        return Self{
            n: IBig::from(int),
            b: 10,
            e,
            p: ((int as f64).abs().log10().floor() + 1.0) as usize
        };
    }
}

impl From<f32> for FBig {
  fn from(num: f32) -> Self {
      let e  = num.log10().ceil() as isize;
      let intstr = num.to_string().replace(".", "");
      let int : i32 = intstr.parse().unwrap();
      return Self{
          n: IBig::from(int),
          b: 10,
          e,
          p: intstr.len()
      };
  }
}

impl From<String> for FBig {
  fn from(s: String) -> Self {
      return FBig::from_str(s.as_str()).unwrap();
  }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{ibig::IBig, fbig::fbig::ZERO};

    use super::FBig;

  #[test]
  fn test_from_impl_for_ints_to_fbig() {
    let expected1 = FBig {
      n : IBig::from(1234),
      b : 10,
      e : 4,
      p : 4
    };
    let expected2 = FBig {
      n : IBig::from(-1234),
      b : 10,
      e : 4,
      p : 4
    };
    
    let num1 = 1234;
    let num2 = -1234;
    let num3 = 0;

    let res = FBig::from(num1 as u16);
    assert_eq!(res, expected1);
    let res = FBig::from(num1 as u32);
    assert_eq!(res, expected1);
    let res = FBig::from(num1 as u64);
    assert_eq!(res, expected1);
    let res = FBig::from(num1 as u128);
    assert_eq!(res, expected1);

    let res = FBig::from(num2 as i16);
    assert_eq!(res, expected2);
    let res = FBig::from(num2 as i32);
    assert_eq!(res, expected2);
    let res = FBig::from(num2 as i64);
    assert_eq!(res, expected2);
    let res = FBig::from(num2 as i128);
    assert_eq!(res, expected2);

    let res = FBig::from(num3);
    assert_eq!(res, ZERO.to_owned());

  }

  #[test]
  fn test_impl_for_floats() {

  }

  #[test]
  fn test_impl_for_from_str() {
    let expected1 = FBig { 
      n : IBig::from(899),
      b : 10,
      e : 3,
      p : 3
    };
    
    let num1 = "899";

    let res = FBig::from_str(num1).unwrap();
    assert_eq!(res, expected1);


  }
}
