// enum Option<T> {
//   Some(T),
//   None,
// }

pub fn main() {
  let x = Some(100); // Option<i64>
  let mut y: Option<&str> = None; // Option<&str>
  dbg!(x);
  dbg!(y);
  y = Some("hello");
  dbg!(y);
  let v: Option<Vec<u64>> = Some(vec![1,2,3]);
  dbg!(&v);

  let xp: *const Option<_> = &x;
  let yp: *const Option<_> = &y;
  let vp: *const Option<_> = &v;
  println!("{:?} {:?} {:?}", xp, vp, yp);

  let ep: *const u64 = &v.as_ref().unwrap()[0];
  println!("{:?}", ep);
  let ep: *const u64 = &v.as_ref().unwrap()[1];
  println!("{:?}", ep);
}
