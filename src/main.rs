#![allow(unused)]
use std::{time::Instant, str::FromStr};

use regex::bytes;
use wasm_bindgen::prelude::*;

use lazy_static::lazy_static;

use rmathrs_dtypes::{ibig::{IBig, parse::non_power_two}, fbig::FBig};



fn main() {
  let curr = Instant::now();
  let max = 1000_000;
  for i in 1..max {
    let s = "123456789123456789121".as_bytes();
    // let num = 
    // println!("s bytess : {s:?}");
    let res = non_power_two::parse3(s, 10);
    // println!("res : {res:?}");
  }  
  println!("parse3 took : {:?}", curr.elapsed());

  let curr = Instant::now();
  for i in 1..max {
    IBig::from_str("123456789123456789121").unwrap();
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