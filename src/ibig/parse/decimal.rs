
use crate::{
  ibig::{
    mul,
    UBig, 
    buffer::{Buffer, self},
    error::ParseError, arch::{
      parse::ParseBytes,
      decimal::{CHUNK_SIZE, MAX_TEN_POWER, TEN_POWS, bytes::*}
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
      let carry = mul::mul_word_in_place_with_carry(&mut buffer, MAX_TEN_POWER, word);
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
      let carry = mul::mul_word_in_place_with_carry(&mut buffer, MAX_TEN_POWER, word);
      if carry != 0 {
          buffer.push(carry);
      }
    }
    return Ok(buffer.into());
  }

  pub fn parse_decimal_bytes_with_fraction(int : &[u8], fraction: &[u8]) -> UBig {
    let groups1 = int.rchunks(CHUNK_SIZE);
    let groups2 = fraction.rchunks(CHUNK_SIZE);
    let mut buffer = Buffer::allocate(groups1.len() + groups2.len());
   
    for group in groups1.rev() {
      let word = DecimalBytes::parse_fast(group);
      let carry = mul::mul_word_in_place_with_carry(&mut buffer, MAX_TEN_POWER, word);
      if carry != 0 {
          buffer.push(carry);
      }
    }

    for group in groups2.rev(){
      let word = DecimalBytes::parse_fast(group);
      let carry = mul::mul_word_in_place_with_carry(&mut buffer, TEN_POWS[group.len()], word);
      if carry != 0 {
          buffer.push(carry);
      }
    }

    return buffer.into();
  }
}