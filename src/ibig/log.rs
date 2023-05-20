use std::ops::Div;

use num_traits::float;

use crate::common::arch::word::{Word, FloatWord};

use super::IBig;

use super::ubig::Repr::{Small, Large};

pub trait Logarithm {
  type Num;

  /// Returns the `&self log to base 2
  fn log2(&self) -> Self::Num;

  /// Returns the `&self log to base 10
  fn log10(&self) -> Self::Num;
}


impl Logarithm for IBig {
    type Num = IBig;

    fn log2(&self) -> Self::Num {
      match self.magnitude().repr() {
        Small(word) => IBig::from((*word as f32).log2() as i32),
        Large(buffer) => IBig::from(buffer.bits()) - 1,
      }
    }

    fn log10(&self) -> Self::Num {
      let log10_2 = (2 as FloatWord).log10();
      let fraction = 1.0 - log10_2;
      match self.magnitude().repr() {
        Small(word) => IBig::from((*word as f32).log10() as i32) + 1,
        Large(_) => {
          let mut ndigit = self.log2().to_f64() * (log10_2); // FIXME convert it to FloatWord, instead to f64

          if ndigit.fract() > fraction {
            ndigit = logx(&self, 10)
          }

          IBig::from(ndigit as Word) + 1
        },
      }
    
    }
}

fn logx(a : &IBig, b : Word) -> FloatWord {
  let base = IBig::from(b);
  let zero = IBig::from(0);

  let mut _a = a.to_owned();
  let mut i = 2;
  let mut lg = 0;

  while _a > base {

    let temp = _a.to_owned() / b.pow(i);
    if temp == zero {
      i >>= 1;
      continue;
    }
    _a = temp;
    lg += i;
    if i < 16 { i *= 2; }

    if i == 0 {break;}
  }

  lg as FloatWord
}


#[cfg(test)]
mod tests {
  use std::str::FromStr;

  use crate::ibig::IBig;

  use super::{Logarithm, logx};

  #[test]
  pub fn test_logx() {
    let a = IBig::from_str("212192719873981982798317982379817").unwrap();
    let b = 10;
    let res = logx(&a, b);
    dbg!(res);
  }

  #[test]
  pub fn test_log2() {
    let expected = IBig::from(5);
    let res = IBig::from(35).log2();
    assert_eq!(expected, res);

    let expected = IBig::from(107);
    let res = IBig::from_str("212192719873981982798317982379817").unwrap().log2();
    assert_eq!(expected, res);

    let expected = IBig::from(122);
    let res = IBig::from_str("8335450802422672859553559685048242436").unwrap().log2();
    assert_eq!(expected, res);
  }
}