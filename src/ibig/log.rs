use super::IBig;

// use crate::ibig::ubig::Repr::{Small, Large};

use super::ubig::Repr::{Small, Large};

pub trait MathLog {
  type Input;
  type Output;

  /// Returns the the `&self log to base` 
  fn log(&self, base : Self::Input) -> Self::Output; 

  /// Returns the `&self log to base e
  fn ln(&self) -> Self::Output;

  /// Returns the `&self log to base 2
  fn log2(&self) -> Self::Output;

  /// Returns the `&self log to base 10
  fn log10(&self) -> Self::Output;
}


impl MathLog for IBig {
    type Input = u32;

    type Output = IBig;

    fn log(&self, base : Self::Input) -> Self::Output {
      todo!()
    }

    fn ln(&self) -> Self::Output {
      todo!()
    }

    fn log2(&self) -> Self::Output {
      match self.magnitude().repr() {
        Small(_) => todo!(),
        Large(_) => todo!(),
      }
    }

    fn log10(&self) -> Self::Output {
        todo!()
    }
}