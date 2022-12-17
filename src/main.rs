#![allow(unused)]
use std::{time::Instant, str::FromStr, str};

use regex::bytes;
use wasm_bindgen::prelude::*;

use lazy_static::lazy_static;

use rmathrs_dtypes::{ibig::{IBig, parse::non_power_two, parse::decimal}, fbig::FBig};



fn main() {
  let max = 1000_000;
  let numstr = "1234567891234567";
  let numbytes = numstr.as_bytes();

  let curr = Instant::now();
  for i in 1..max {
    let res = IBig::from_str(numstr).unwrap();
    // println!("res : {res}");

  }

  println!("Ibig took : {:?}", curr.elapsed());

  // init_ibig_perf();
  // init_fbig_perf();
}


fn init_fbig_perf() {
  let curr = Instant::now();
  let res = FBig::from_str("12121").unwrap();
  println!("{:?}", res);
  println!("Took {:?}", curr.elapsed());
}
fn init_ibig_perf() {
  let curr = Instant::now();
  let res = IBig::from_str("12121").unwrap();
  println!("{:?}", res);
  // for i in 1..1000_000 {
  //   // let st = i.to_string() + "12121";
  //   IBig::from_str("12121").unwrap();
  // }
  println!("Took {:?}", curr.elapsed());
}