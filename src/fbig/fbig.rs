use wasm_bindgen::prelude::wasm_bindgen;

use crate::ibig::IBig;

/// Real 'd' represented as :
///    d = n * b ^ -e
/// 
/// e.g. 
///  0.323928398 = 0.323928398 * 10 ^ 0 = 0.3239283980 * 10 ^ 0
///      here n = 323928398, b = 10, e = 0, p = 9
///      here n = 3239283980, b = 10, e = 0, p =10
///  787600000 = 0.7876 * 10 ^ 9 = 0.78760 * 10 ^ 8
///      here n = 7876, b = 10, e = 9, p = 4
///      here n = 78760, b = 10, e = 9, p = 5
#[wasm_bindgen]
#[derive(Debug)]
pub struct FBig {
  n : IBig,
  b : usize,
  e : isize,
  p : usize
}

#[wasm_bindgen]
impl FBig {
    pub fn new(n : IBig, b : usize, e : isize, p : usize) -> FBig {
      return FBig {
        n, b, e, p
      };
    }
}


