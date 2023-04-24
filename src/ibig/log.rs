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
        Small(word) => IBig::from((*word as f32).log2() as i32) + 1,
        Large(buffer) => IBig::from(buffer.bits()),
      }
    }

    fn log10(&self) -> Self::Num {
      let log10_2 = (2 as FloatWord).log10();
      let fraction = 1.0 - log10_2;
      match self.magnitude().repr() {
        Small(word) => IBig::from((*word as f32).log10() as i32) + 1,
        Large(_) => {
          let ndigit = self.log2().to_f64() * (log10_2); // FIXME convert it to FloatWord, instead to f64
          
          if ndigit.fract() > fraction {
            todo!("Boundry condition not implemented")
          }

          IBig::from(ndigit as Word) + 1
        },
      }
    
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::ibig::IBig;

    use super::Logarithm;

  #[test]
  pub fn test_log2() {
    let expected = IBig::from(6);
    let res = IBig::from(35).log2();
    assert_eq!(expected, res);

    let expected = IBig::from(108);
    let res = IBig::from_str("212192719873981982798317982379817").unwrap().log2();
    assert_eq!(expected, res);
  }
}