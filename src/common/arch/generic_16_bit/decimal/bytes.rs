use std::{string::ParseError, convert::Infallible, num::ParseIntError};

use super::super::parse::ParseBytes;

static TEN_POW : [u64; 8] = [1, 10, 100, 1000, 10000, 1000000, 10000000, 100000000];

/// initial inspiration -> https://rust-malaysia.github.io/code/2020/07/11/faster-integer-parsing.html

// IDEA 
//                            0x04030201
// +-----------------------------|--------------------+
// | 1 byte mask trick           v  v--- one byte     |
// |                         +-+-+-+-+                |
// |                         |1|2|3|4|                |
// |                         +-+-+-+-+                |
// |                           |   |                  |
// |            0x000f000f  <--+   +-->  0x0f000f00   |
// |                 |                        |       |
// |                 v                        v       |
// |               0 2 0 4                  1 0 3 0   |
// |       (>> 8)  2 0 4 0          (* 10) 10 030 0   |
// |                 |                        |       |
// |                 +-----------+------------+       |
// |                         (+) |                    |
// |                             v v--- two bytes     |
// |                          +--+--+                 |
// |                          |12|34|                 |
// |                          +--+--+                 |
// +-----------------------------|---------------------
//                               |
//                     +--------------------+
//                     | 2 bytes mask trick |
//                     +--------------------+
//                               |
//                               v  v--- four bytes
//                            +------+
//                            | 1234 |
//                            +------+
/// 1-byte mask trick (works on 4 pairs of single digits)
///  0x0f000f000f000f00          0x000f000f000f000f
/// 2-byte mask trick (works on 2 pairs of two digits)
///  0x00ff000000ff0000          0x000000ff000000ff
/// 4-byte mask trick (works on a pair of four digits)
///  0x0000ffff00000000          0x000000000000ffff
/// Trick 
///   let s = s[..2].as_ptr() as *const _;
///  let mut chunk : u16 = 0;
///  unsafe {
///    std::ptr::copy_nonoverlapping(s, &mut chunk, std::mem::size_of_val(&chunk));
///  };


// This


pub struct DecimalBytes;

impl ParseBytes for DecimalBytes {
    fn parse(b : &[u8]) -> Result<u32, ParseIntError> {
      if b.len() > 8 {panic!("Byte len should be < 8, priovided {}", b.len())};
      return std::str::from_utf8(b).unwrap().parse();
    }
    fn parse_str(s : &str) -> Result<u32, ParseIntError> {
      return s.parse::<u32>();
    }
    fn parse_byte(b : &u8) -> u32 {
      return (b & 0xf) as u32;
    }

    fn parse_fast(b : &[u8]) -> u32 {
      unsafe {
        return std::str::from_utf8_unchecked(b).parse::<u32>().unwrap();
      }
    }

    fn parse_assert(b : &[u8]) ->u32 {
      if b.len() > 8 {panic!("Byte len should be < 8, priovided {}", b.len())};
      return std::str::from_utf8(b).unwrap().parse().unwrap();
    }

    fn parse_str_fast(s : &str) -> u32 {
      return s.parse().unwrap();
    }

    fn parse_str_assert(s : &str) -> u32 {
      if s.len() > 16 {panic!("Byte len should < 16, priovided {}", s.len())};
      return s.parse().unwrap();
    }

    fn parse_4_bytes(b: &[u8]) -> u16 { // no need to benchmark this, to be used later
      let (upper_digits, lower_digits) = b.split_at(2);
      Self::parse_2_bytes(upper_digits) * 100 + Self::parse_4_bytes(lower_digits)
    }

    fn parse_2_bytes(b : &[u8]) -> u16 {
      let b = b.as_ptr() as *const _;
      let mut chunk : u16 = 0;
      unsafe {
        std::ptr::copy_nonoverlapping(b, &mut chunk, std::mem::size_of_val(&chunk));
      };
    
      let lower_digits = (chunk & 0x0f00) >> 8;
      let upper_digits = (chunk & 0x000f) * 10;
      let chunk = (lower_digits + upper_digits) as u32;
    
      return chunk;
    }

    fn parse_8_bytes(b : &[u8]) -> u16 {
      let (upper_digits, lower_digits) = b.split_at(4);
      Self::parse_4_byte(upper_digits) * 10000 + Self::parse_4_byte(lower_digits)
    }
  }