fn main() {
  let mut foo = 42;
  println!("{:p}", &foo);

  let f = &mut foo;
  println!("{:p}", &f);
  println!("{}", f);

  let bar = *f;
  println!("{}", bar);
  println!("{}", f);

  *f = 13;
  println!("{}", bar);
  println!("{}", foo);
}
