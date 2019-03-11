#![allow(dead_code)]
#![allow(unused_variables)]

// unsigned integers: u8, u16, u32, u64, u128, usize
// signed integers:   i8, i16, i32, i64, i128, size
// floats:            f32, f64
// bool:              bool

// mod div_option;
// mod div_result;
// mod enums;
// mod option;
// mod string;
// mod traits;
// mod serialization deserialization;

// strings:
// - reads from standard input
// - prints ascii characters to stdout
//
// extras:
// - only runs of minimum length
// - read from files passed as paths
// - take run length on command line
//

use std::io::{self, prelude::*};

mod error;

fn main() {
  // enums::main();
  // option::main();

  for r in io::stdin().bytes() {
    println!("{:?}", r);
  }
}
