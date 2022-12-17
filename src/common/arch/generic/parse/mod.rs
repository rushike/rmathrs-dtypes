use std::num::ParseIntError;

use super::word::Word;

pub trait ParseBytes {
  fn parse(b : &[u8]) -> Result<Word, ParseIntError>;
  fn parse_fast(b : &[u8]) -> Word;
  fn parse_assert(b : &[u8]) ->Word;
  fn parse_byte(b : &u8) -> Word;
  fn parse_2_bytes(b : &[u8]) -> Word;
  fn parse_4_bytes(b : &[u8]) -> Word;
  fn parse_8_bytes(b : &[u8]) -> Word;
  fn parse_str(s : &str) -> Result<Word, ParseIntError>;
  fn parse_str_fast(s : &str) -> Word;
  fn parse_str_assert(s : &str) -> Word;
}
