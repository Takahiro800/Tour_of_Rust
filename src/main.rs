struct Foo {
  x: i32,
  y: i32,
}

fn do_something(a: &Foo) -> &i32 {
  return &a.x;
}
fn main() {
  let mut foo = Foo { x: 42, y: 5 };
  let x = &mut foo.x;
  println!("{}", x);
  *x = 13;
  println!("{}", x);

  // x はここでドロップされるため、不変な参照が作成可能
  let y = do_something(&foo);
  println!("{}", y);
  // y はここでドロップ
  // foo はここでドロップ
}
