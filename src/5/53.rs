struct Foo {
  x: i32,
}

//  これだと、lifetimeが明示されてないのでだめ
// fn do_something(foo: Foo) -> &i32 {
fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
  return &foo.x;
}
fn main() {
  let foo = Foo { x: 42 };
  let x = &foo.x;
  println!("{}", x);
  // x はここでドロップされるため、不変な参照が作成可能
  let y = do_something(&foo);
  println!("{}", y);
}
