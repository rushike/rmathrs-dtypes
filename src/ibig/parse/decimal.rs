
use crate::{
  ibig::{
    mul,
    UBig, 
    buffer::{Buffer, self},
    error::ParseError, arch::{
      parse::ParseBytes,
      decimal::{CHUNK_SIZE, TEN_POWER, bytes::*}
    }
  }
};

use alloc::vec;

impl UBig {
  pub fn parse_decimal_str(num : &str) -> Result<UBig, ParseError> {

    let bytes = num.as_bytes();
    let mut num_start_index = 0;
    while bytes[num_start_index] == b'0' {
        num_start_index += 1;
    }

    let groups = bytes[num_start_index..].rchunks(CHUNK_SIZE);
    let mut buffer = Buffer::allocate(groups.len());
    for group in groups.rev() {
      let word = DecimalBytes::parse_fast(group);
      let carry = mul::mul_word_in_place_with_carry(&mut buffer, TEN_POWER, word);
      if carry != 0 {
          buffer.push(carry);
      }
    }
    return Ok(buffer.into());
  }

  pub fn parse_decimal_bytes(num : &[u8]) -> Result<UBig, ParseError> {
    let groups = num.rchunks(CHUNK_SIZE);
    let mut buffer = Buffer::allocate(groups.len());
    for group in groups.rev() {
      let word = DecimalBytes::parse_fast(group);
      let carry = mul::mul_word_in_place_with_carry(&mut buffer, TEN_POWER, word);
      if carry != 0 {
          buffer.push(carry);
      }
    }
    return Ok(buffer.into());
  }
}