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
    let mut iend = bytes.len(); 
    let mut fstart = iend;
    let fend = bytes.len();

    for i in istart..fend{ 
      if bytes[i] == b'.' {
        iend = i;
        fstart = i + 1;
        break;
      }
    }
    let magnitude = UBig::parse_decimal_bytes_with_fraction(&bytes[istart..iend], &bytes[fstart..fend]);

    // let magnitude = UBig::parse_decimal_bytes(&resbytes).unwrap();
    let e = (iend - istart) as isize;
    let n = IBig::from_sign_magnitude(sign, magnitude);
    let b = 10;
    let p = fend - istart;
    return  Ok(FBig::new(n, b, e, p));
    
  }
}

/// Converts a byte (ASCII) representation of a digit to its value.
pub(crate) fn digit_from_utf8_byte(byte: &u8, radix: Digit) -> Option<Digit> {
  let res = match byte {
      b'0'..=b'9' => (byte - b'0') as Digit,
      b'a'..=b'z' => (byte - b'a') as Digit + 10,
      b'A'..=b'Z' => (byte - b'A') as Digit + 10,
      _ => return None,
  };
  if res < radix {
      Some(res)
  } else {
      None
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