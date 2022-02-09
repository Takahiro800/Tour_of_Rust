struct Foo {
  x: i32,
}
fn do_something(f: &mut Foo) {
  f.x += 1;
  println!("{:p}", f);
  // f への可変な参照はここでドロップ
}
fn main() {
  let mut foo = Foo { x: 42 };
  println!("{}", foo.x);
  do_something(&mut foo);
  // 関数 do_something で可変な参照はドロップされるため、
  // 別の参照を作ることが可能
  println!("{}", foo.x);
  do_something(&mut foo);
  // foo はここでドロップ
}
