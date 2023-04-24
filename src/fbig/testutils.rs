use std::str::FromStr;

use crate::fbig::FBig;


#[derive(Debug, Clone)]
pub enum TestInput {
  Int8(i8),
  Uint8(u8),
  Int16(i16),
  Uint16(u16),
  Int32(i32),
  Uint32(u32),
  Int64(i64),
  Uint64(u64),
  Int128(i128),
  Uint128(u128),
  Float32(f32),
  Float64(f64),

  Str(String),
}

impl From<TestInput> for FBig
{
 fn from(a : TestInput) -> FBig{
    match a {
        TestInput::Int8(v) => FBig::from(v),
        TestInput::Uint8(v) => FBig::from(v),
        TestInput::Int16(v) => FBig::from(v),
        TestInput::Uint16(v) => FBig::from(v),
        TestInput::Int32(v) => FBig::from(v),
        TestInput::Uint32(v) => FBig::from(v),
        TestInput::Int64(v) => FBig::from(v),
        TestInput::Uint64(v) => FBig::from(v),
        TestInput::Int128(v) => FBig::from(v),
        TestInput::Uint128(v) => FBig::from(v),
        TestInput::Float32(v) => FBig::from(v),
        TestInput::Float64(v) => FBig::from(v),
        TestInput::Str(v) => FBig::from(v),
    }
  }
}

pub fn to_fbig_vector(vec: Vec<TestInput>) -> Vec<FBig> {
  vec
  .iter()
  .map(|x| x.clone().into())
  .collect::<Vec<FBig>>()
}


pub fn apply(ops : fn(FBig, FBig) -> FBig, a : Vec<FBig>, b : Vec<FBig>) -> Vec<FBig> {
  a.iter().zip(b.iter())
    .enumerate()
    .map(|(_, (x, y))| ops(x.clone(), y.clone()))
    .collect::<Vec<FBig>>()
}

