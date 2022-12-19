
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use core::str::FromStr;

use rmathrs_dtypes::{fbig::FBig, ibig::IBig};

fn fbig_init_from_str() {
  FBig::from_str("12345.6789").unwrap();
}

fn ibig_init_from_str() {
  IBig::from_str("123456789").unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
  // c.bench_function("ibig init from str", |b| b.iter(|| ibig_init_from_str()));
  c.bench_function("fbig init from str", |b| b.iter(|| fbig_init_from_str()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);