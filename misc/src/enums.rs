use std::{f64, mem};
use std::fmt::{self, Debug, Formatter};

#[derive(Debug)]
enum Color {
  Red,
  Green,
  Blue,
}

// Color
//  0
// [x]

mod fake_enum {
  static COLOR_RED: u8 = 0;
  static COLOR_GREEN: u8 = 1;
  static COLOR_BLUE: u8 = 1;
}

#[derive(Clone, Copy)]
enum Shape {
  Circle(f64),
  Rectangle(f64, f64),
}

impl Shape {
  fn area(self) -> f64 {
    use Shape::*;
    match self {
      Circle(radius) => f64::consts::PI * radius * radius,
      Rectangle(width, height) => width * height,
    }
  }
}

impl Debug for Shape {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use Shape::*;
    match self {
      Circle(radius) => write!(f, "I'M A CIRCLE HUR DUR: {}", radius),
      Rectangle(width, height) => write!(f, "I'M A RECTANGLE HUR DUR: {} x {}", width, height),
    }
  }
}

#[derive(Debug)]
enum Foo {
  Bar(f64),
  Baz(f64, f64),
}

// Shape
//
// [d___xxxxxxxxyyyyyyyy]
// [0   RRRRRRRR        ]
// [1   WWWWWWWWHHHHHHHH]

pub fn main() {
  let c = Color::Red;
  dbg!(c);

  let circle = Shape::Circle(2.0);
  dbg!(circle);

  let rect = Shape::Rectangle(1.0, 10.0);
  dbg!(rect); // === println!("rect = {:?}", rect);

  unsafe {
    let garbage: [u8; 24] = mem::transmute(rect);
    dbg!(garbage);
  }

  println!("circle area: {}", circle.area());
  println!("rect area:   {}", rect.area());
}
