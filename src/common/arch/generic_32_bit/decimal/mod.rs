pub const CHUNK_SIZE : usize = 8;

pub const MAX_TEN_POWER : u32 = 100_000_000; 

pub const TEN_POWS : [u32; 9] = [
  1, 10, 100, 1000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000
];

pub mod bytes;