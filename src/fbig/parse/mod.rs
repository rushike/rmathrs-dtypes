use core::{str::FromStr, num};
use std::ops::ControlFlow;

use regex::{Regex, bytes};

use lazy_static::lazy_static;

use crate::{
  fbig::FBig, 
  ibig::{
    error::ParseError,
    IBig,
    radix::{self, Digit},
    sign::Sign::{*, self},
    ubig::UBig, buffer::Buffer, parse::non_power_two,
  }
};


impl FromStr for FBig {
  type Err = ParseError;
  
  fn from_str(s: &str) -> Result<FBig, ParseError> {
      FBig::from_str_radix(s, 10)
  }
}

impl FBig {
    // #[wasm_bindgen]
    /// //TODO: Need to implement below function for all radix.
    pub fn from_str_radix(num : &str, radix : usize) -> Result<FBig, ParseError> {
      match radix {
          10 => return FBig::from_decimal_str(num),
          _ => Err(ParseError::InvalidDigit) // ("Non decimal string parsing not implemented")
      }
    }

  /// This parases the floating decimal string of format <sign>?<integer>.<fraction>
  /// It assumes string contains digits from 0..9 only.
  /// 
  /// Example
  /// ```
  /// // from_decimal_str("123.123");
  /// ``` 
  fn from_decimal_str(num : &str) -> Result<FBig, ParseError> {
    let numbytes = num.as_bytes();
    let sign;
    let numindex = match numbytes[0] {
      b'-' => {
          sign = Negative;
          1
      },
      b'+' => {
          sign = Positive;
          1
      },
      _ => {
          sign = Positive;
          0
      }
    };
    return FBig::parse_decimal_bytes(numbytes, sign, numindex);

  }

  fn parse_decimal_bytes(bytes : &[u8], sign : Sign, mut start : usize) -> Result<FBig, ParseError>{
    
    for b in &bytes[start..bytes.len() - 1] {
      if *b != b'0' { break; }
      start += 1;
    }

    let istart = start;
    let mut nstart = start;
    let mut iend = bytes.len(); 
    let mut fstart = iend;
    // let mut search_for_fraction_leading_zeros = false; // will search for fractional leading zeros only after the occurrence of '.'
    let fend = bytes.len();

    for i in istart..fend{ 
      // check if integer part of number ends
      if bytes[i] == b'.' {
        iend = i;
        fstart = i + 1;
      }
      // check if integer part is non zero and assign fstart and breaks
      else if nstart < iend {
        // break;
      }
      // below if integer part is zero and searches for fractional leading zeros, i.e. zeros after decimal point (.) like 0.02, 0.00343 etc these numbers have fraction leading zeros 
      else if nstart >= iend && bytes[i] != b'0' {
        fstart = i;
        nstart = i - 1;
        break;
      }
    }

    // dbg!(istart);
    // dbg!(nstart);
    // dbg!(iend);
    // dbg!(fstart);
    // dbg!(fend);

    // dbg!(&bytes[istart..iend]);
    // dbg!(&bytes[fstart..fend]);

    let magnitude = UBig::parse_decimal_bytes_with_fraction(&bytes[istart..iend], &bytes[fstart..fend]);

    // let magnitude = UBig::parse_decimal_bytes(&resbytes).unwrap();
    let e = iend as isize - nstart as isize ;
    let n = IBig::from_sign_magnitude(sign, magnitude);
    let b = 10;
    let p = fend - fstart + iend - istart;
    return  Ok(FBig::new(n, b, e, p));
    
  }
}

#[cfg(test)]
mod parse {
    use std::str::FromStr;

    use crate::{fbig::FBig, ibig::IBig};

  #[test]
  fn parse_decimal() {
    let base = 10;

    let num = "899";
    let expected = FBig::new(IBig::from_str(num).unwrap(), base, 3, 3);
    let res = FBig::from_str_radix(num, base).unwrap();
    assert_eq!(res, expected);


    let num = "123456789123456789121";
    let expected = FBig::new(IBig::from_str(num).unwrap(), base, 21, 21);
    let res = FBig::from_str_radix(num, base).unwrap();
    assert_eq!(res, expected);
    
    let num = "-0.3232";
    let expected = FBig::new(IBig::from_str("-3232").unwrap(), base, 0, 5);
    let res = FBig::from_str_radix(num, base).unwrap();
    assert_eq!(res, expected);
  }
}