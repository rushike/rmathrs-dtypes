use std::{str::FromStr};

use wasm_bindgen::prelude::*;

use crate::ibig::{IBig, ops::Abs};

#[wasm_bindgen]
impl IBig {

  /* */
  pub fn copy(&self) -> IBig {
    return self.clone();
  }
  #[wasm_bindgen(js_name=clone)]
  pub fn clone_wasm(&self) -> IBig {
    return self.clone();
  }
  /* Init Functions */
  pub fn from_i32(n : i32)-> IBig {
    return  IBig::from(n);
  }

  #[wasm_bindgen(js_name = from_str)]
  pub fn from_str_wasm(n : &str) -> IBig {
    return IBig::from_str(n).unwrap();
  }


  /* tostring methods */
  #[wasm_bindgen(js_name = toString)]
  pub fn to_string_wasm(&self) -> String {
    return IBig::to_string(&self);
  }

  #[wasm_bindgen(js_name = toStringRadix)]
  pub fn to_string_radix(&self, radix : u32) -> String {
    return IBig::in_radix(&self, radix).to_string();
  }

  /* Arithmetic Ops */
  pub fn plus(&self, other : &IBig) -> IBig {
    return self + other;
  }

  #[wasm_bindgen(js_name = add)]
  pub fn add_wasm(&self, other : &IBig) -> IBig {
    return self + other;
  }

  pub fn minus(&self, other : &IBig) -> IBig {
    return self - other;
  }

  #[wasm_bindgen(js_name = sub)]
  pub fn sub_wasm(&self, other : &IBig) -> IBig {
    return self + other;
  }

  pub fn times(&self, other : &IBig) -> IBig {
    return  self * other;
  }

  #[wasm_bindgen(js_name = mul)]
  pub fn mul_wasm(&self, other : &IBig) -> IBig {
    return  self * other;
  }

  pub fn over(&self, other : &IBig) -> IBig {
    return self / other;
  }

  #[wasm_bindgen(js_name = div)]
  pub fn div_wasm(&self, other : &IBig) -> IBig {
    return self / other;
  }

  #[wasm_bindgen(js_name = mod)]
  pub fn mod_wasm(&self, other : &IBig) -> IBig {
    return  self % other;
  }

  #[wasm_bindgen(js_name = abs)]
  pub fn abs_wasm(&self) -> IBig {
    return  self.abs();
  }

  /* Comparision Operators */
  #[wasm_bindgen(js_name = eq)]
  pub fn eq_wasm(&self, other : &IBig) -> bool {
    return self == other;
  }

  #[wasm_bindgen(js_name = ne)]
  pub fn ne_wasm(&self, other : &IBig) -> bool {
    return self != other;
  }

  #[wasm_bindgen(js_name = ge)]
  pub fn ge_wasm(&self, other : &IBig) -> bool {
    return  self >= other;
  }

  #[wasm_bindgen(js_name = le)]
  pub fn le_wasm(&self, other : &IBig) -> bool {
    return  self <= other;
  }

  #[wasm_bindgen(js_name = gt)]
  pub fn gt_wasm(&self, other : &IBig) -> bool {
    return  self > other;
  }

  #[wasm_bindgen(js_name = lt)]
  pub fn lt_wasm(&self, other : &IBig) -> bool {
    return  self < other;
  }
}