#![allow(unused)]
use std::{time::Instant, str::FromStr, str};

use regex::bytes;
use wasm_bindgen::prelude::*;

use lazy_static::lazy_static;

use rsdtypes::{ibig::{IBig, parse::non_power_two, parse::decimal}, fbig::FBig};



fn main() {
  // init_ibig_perf();
  init_fbig_perf();
}


fn init_fbig_perf() {
  let curr = Instant::now();
  let res = FBig::from_str("12345.6789").unwrap();
  println!("from str : {:?}", res);
  let res = FBig::from_str("123456789123456789.8888123456789123456789123456789").unwrap();
  println!("from str : {:?}", res);
  let res = FBig::from(12345);
  println!("from i32 : {:?}", res);
  let res = FBig::from(12345.6789);
  println!("from f64 : {:?}", res);
  println!("Took {:?}", curr.elapsed());
}
fn init_ibig_perf() {
  let curr = Instant::now();
  let res = IBig::from_str("12121").unwrap();
  println!("str : {:?}", res);
  println!("Took {:?}", curr.elapsed());
}