use core::{str::FromStr, num};

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
    pub fn from_str_radix(num : &str, radix : usize) -> Result<FBig, ParseError> {
      match radix {
          10 => return FBig::from_decimal_str(num, 10),
          _ => Err(ParseError::InvalidDigit) // ("Non decimal string parsing not implemented")
      }
    }


    fn from_decimal_str1(num : &str, radix : usize) -> Result<FBig, ParseError> {
      lazy_static! { 
        static ref NUM_STRING_MATCH_EXP: Regex = Regex::new(r"(?P<sign>[+-]?)([0]*)(?P<integer>\d+)?(\.(?P<fraction>(?P<leading_frac_zeros>[0]*)?\d+))?(e(?P<exp>[+-]?\d+))?").unwrap();
      }
      let captures = NUM_STRING_MATCH_EXP.captures(num).unwrap();

      let sign = match captures.name("sign") {
          Some(s) => s.as_str(),
          None => "+"
      };

      let integer = match captures.name("integer") {
          Some(i) => i.as_str(),
          None => "0"
      };

      let fraction = match captures.name("fraction") {
          Some(f) => f.as_str(),
          None => ""
      };

      let exp = match captures.name("exp") {
          Some(e) => e.as_str().parse().unwrap(),
          None => 0
      };

      let leading_frac_zeros = match captures.name("leading_frac_zeros") {
          Some(lfz) => lfz.as_str().len() as isize,
          None => 0
      };

      let numstr = format!("{}{}{}", sign, integer, fraction);
      let p = if integer == "0" {numstr.len() - 2} else {numstr.len() - 1};
      let b = 10;
      let n = IBig::from_str(numstr.as_str()).unwrap();
      let e = if integer == "0" {-leading_frac_zeros + exp} else {integer.len() as isize + exp};

      return Ok(FBig::new(n, b, e, p));
    }

  fn from_decimal_str(num : &str, radix : usize) -> Result<FBig, ParseError> {
    let numbytes = num.trim().as_bytes();
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
    return FBig::parse_decimal_bytes(numbytes, sign, numindex, 10);

  }

  fn parse_decimal_bytes(bytes : &[u8], sign : Sign, start : usize, radix : Digit) -> Result<FBig, ParseError>{
    let mut num_start_index = 0;
    while bytes[num_start_index] == b'0' {
        num_start_index += 1;
    }

    let mut resbytes = Vec::with_capacity(bytes.len());
    let mut e = 0;
    let mut len = 0;
    for (i, byte) in bytes[start..].iter().enumerate() {
      if *byte == b'.' {
        e = i;
        continue;
      }
      resbytes.push(*byte);
      len += 1;
    }
    let magnitude = non_power_two::parse2(&resbytes[..len], radix).unwrap();
    let n = IBig::from_sign_magnitude(sign, magnitude);
    let b = 10;
    let p = resbytes.len();
    return  Ok(FBig::new(n, b, e as isize, p));
    // // let groups = bytes.rchunks(radix_info.digits_per_word);
    // let groups = bytes.rchunks(4);
    // let mut buffer = Buffer::allocate(bytes.len());
    // for byte in bytes{
    //     match digit_from_utf8_byte(byte, radix) {
    //         Some(word)=>{
    //           buffer.push(word)
    //         }
    //         None => {
    //           return Err(ParseError::InvalidDigit);
    //         }
    //     }
    // }
    // Err(ParseError::InvalidDigit)
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
    use crate::fbig::FBig;

  #[test]
  fn parse_decimal() {
    // FBig::from_str_radix("0.3232", 10);
    // println!(":):)");
    let res1 = FBig::from_str_radix("123456789123456789121", 10);
    println!(":):) {:?}", res1);
    let res2 = FBig::from_str_radix("-0.3232", 10);
    println!(":):) {:?}", res2);

  }
}