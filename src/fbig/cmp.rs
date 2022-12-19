use std::cmp::Ordering;

use crate::ibig::{IBig, self};

use crate::ibig::shift::decimal::DecimalShift;

use super::FBig;

impl PartialEq for FBig {
  fn eq(&self, other: &Self) -> bool {
      return self.b == other.b
        && if self.e == other.e { 
          if self.p > other.p { 
            self.n == other.n.shld(self.p - other.p)
          } else if self.p < other.p {
            other.n == self.n.shld(other.p - self.p)
          }
          else {
              self.n == other.n
          }
        } else {
          false
        };
  }
}

impl PartialOrd for FBig {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.b.partial_cmp(&other.b) {
            Some(Ordering::Equal) => {}
            _ => return panic!("Comparing FBig with different bases not supported"),
        }
        match self.e.partial_cmp(&other.e) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        
        if self.p > other.p { 
          return self.n.partial_cmp(&other.n.shld(self.p - other.p));
        } 
        else if self.p < other.p {
          return other.n.partial_cmp(&self.n.shld(other.p - self.p));
        }
        else {
          return self.n.partial_cmp(&other.n)
        }
    }
}

impl Ord for FBig {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fbig_cmp() {
      let a = FBig::from(1234.56789);
      let b = FBig::from(987.654321);
      let c = FBig::from(1234.56789);

      assert_eq!(a < b, false);
      assert_eq!(a > b, true);
      assert_eq!(a == c, true);
    }
}