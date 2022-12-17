static TEN_POW : [u64; 8] = [1, 10, 100, 1000, 10000, 1000000, 10000000, 100000000];
//                             0x04030201
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

pub fn parse_1_byte(s: &[u8]) -> u64{
  return (s[0] & 0xf) as u64;
}

pub fn parse_2_byte(s : &[u8]) -> u64 {
  let s = s[..2].as_ptr() as *const _;
  let mut chunk : u16 = 0;
  unsafe {
    std::ptr::copy_nonoverlapping(s, &mut chunk, std::mem::size_of_val(&chunk));
  };

  let lower_digits = (chunk & 0x0f00) >> 8;
  let upper_digits = (chunk & 0x000f) * 10;
  let chunk = (lower_digits + upper_digits) as u64;

  return chunk;
}

pub fn parse_4_byte(s : &[u8]) -> u64 {
  let size = s.len();

  let s = if size > 4 {&s[..4]} else {s};

  let s = s.as_ptr() as *const _;
  let mut chunk : u32 = 0;
  unsafe {
    std::ptr::copy_nonoverlapping(s, &mut chunk, size);
  };

  let lower_digits = (chunk & 0x0f000f00) >> 8;
  let upper_digits = (chunk & 0x000f000f) * 10;
  let chunk = lower_digits + upper_digits;

  // 2-byte mask trick (works on 2 pairs of two digits)
  let lower_digits = (chunk & 0x00ff0000) >> 16;
  let upper_digits = (chunk & 0x000000ff) * 100;
  let chunk = (lower_digits + upper_digits) as u64;
  return chunk / TEN_POW[4 - size];
}

pub fn parse_8_bytes(s: &[u8]) -> u64 { // no need to benchmark this, to be used later

  let s = s.as_ptr() as *const _;
  let mut chunk = 0;
  unsafe {
      std::ptr::copy_nonoverlapping(s, &mut chunk, 8);
  }

  // 1-byte mask trick (works on 4 pairs of single digits)
  let lower_digits = (chunk & 0x0f000f000f000f00) >> 8;
  let upper_digits = (chunk & 0x000f000f000f000f) * 10;
  let chunk = lower_digits + upper_digits;

  // 2-byte mask trick (works on 2 pairs of two digits)
  let lower_digits = (chunk & 0x00ff000000ff0000) >> 16;
  let upper_digits = (chunk & 0x000000ff000000ff) * 100;
  let chunk = lower_digits + upper_digits;

  // 4-byte mask trick (works on a pair of four digits)
  let lower_digits = (chunk & 0x0000ffff00000000) >> 32;
  let upper_digits = (chunk & 0x000000000000ffff) * 10000;
  let chunk = lower_digits + upper_digits;

  return chunk
}

pub fn parse_8_bytes_flex(s: &[u8]) -> u64 { // no need to benchmark this, to be used later
  let size = s.len();

  let s = s.as_ptr() as *const _;
  let mut chunk = 0;
  unsafe {
      std::ptr::copy_nonoverlapping(s, &mut chunk, size);
  }

  // 1-byte mask trick (works on 4 pairs of single digits)
  let lower_digits = (chunk & 0x0f000f000f000f00) >> 8;
  let upper_digits = (chunk & 0x000f000f000f000f) * 10;
  let chunk = lower_digits + upper_digits;

  // 2-byte mask trick (works on 2 pairs of two digits)
  let lower_digits = (chunk & 0x00ff000000ff0000) >> 16;
  let upper_digits = (chunk & 0x000000ff000000ff) * 100;
  let chunk = lower_digits + upper_digits;

  // 4-byte mask trick (works on a pair of four digits)
  let lower_digits = (chunk & 0x0000ffff00000000) >> 32;
  let upper_digits = (chunk & 0x000000000000ffff) * 10000;
  let chunk = lower_digits + upper_digits;

  return chunk / TEN_POW[8 - size];
}


pub fn parse_16_bytes(s: &[u8]) -> u64 {
  // println!("{:?}",s);
  let (upper_digits, lower_digits) = s.split_at(8);
  parse_8_bytes(upper_digits) * 100000000 + parse_8_bytes(lower_digits)
}