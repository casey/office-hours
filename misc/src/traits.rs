struct Foo {}

mod bar {
  pub trait Bar {
    fn f(self);
  }
}

mod baz {

  pub trait Baz {
    fn f(self);
  }
}

impl bar::Bar for Foo {
  fn f(self) {
    println!("Bar!");
  }
}

impl baz::Baz for Foo {
  fn f(self) {
    println!("Baz!");
  }
}

// use bar::Bar;
// use baz::Bar;

fn main() {
  let foo = Foo {};
  foo.f();
}
