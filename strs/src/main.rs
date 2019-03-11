#![allow(dead_code)]
#![allow(unused_variables)]

// extras:
// - read from files passed as paths
// - take run length on command line
// - do utf8 to

use structopt::StructOpt;

mod common;
mod error;

use crate::common::*;

#[derive(StructOpt)]
#[structopt(name = "strs")]
struct Opt {
  #[structopt(name = "N", short = "n", long = "run-length", default_value = "4")]
  run_length: usize,

  // #[struct(name = FILE, short "f", long I
}

/*
#[derive(Serialize, Deserialize)]
struct Foo {
  bar: String,
  baz: Option<f46>,
  qux: Qux,
}

#[derive(Serialize, Deserialize)]
enum Qux {
  A,
  B,
  C(u64, String),
  V(Vec<u64>),
}

  let mut f: Foo = sedre_json::from_str("SOME JSON")?;

  // modify f
  println!("{}", serde_json::to_string(&f));

*/

fn main() -> Result<(), Error> {


  let opt = Opt::from_args();

  let mut run = String::new();

  for byte in io::stdin().bytes() {
    let byte = byte?;

    if byte >= 32 && byte <= 126 {
      let character = byte as char;
      run.push(character);
    } else {
      if run.len() >= opt.run_length {
        println!("{}", run);
      }
    
      run.clear();
    }
  }

  if run.len() >= opt.run_length {
     println!("{}", run);
  }

  Ok(())
}
