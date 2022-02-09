struct Foo<'a> {
  i: &'a i32,
}

struct Bar {
  i: i32,
}
fn main() {
  let x = 42;
  let foo = Foo { i: &x };
  println!("{}", foo.i);

  let y = 13;
  let bar = Bar { i: y };
  println!("{}", bar.i);
}
