use core::str::FromStr;

use regex::Regex;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::fbig::FBig;

// pub static NumStringMatchExp : Lazy<Regex> = Lazy::new(|| Regex::new(r"([+-]?)([0]*)(\d+)?(\.(([0]*)?\d+))?(e([+-]?\d+))?").unwrap());

#[wasm_bindgen]
impl FBig {
  #[wasm_bindgen(js_name = from_str)]
  pub fn from_str_wasm(num : &str) -> FBig {
    return FBig::from_str(num).unwrap();
  }
}

#[cfg(test)]
mod wasm {

}