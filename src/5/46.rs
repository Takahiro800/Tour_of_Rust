struct Foo {
  x: i32,
}

fn do_something(f: Foo) {
  println!("{}", f.x);
}
fn main() {
  let foo = Foo { x: 42 };
  println!("{}", foo.x);
  // foo の所有権は do_something に移動
  do_something(foo);
  // foo は使えなくなる
  // println!("{}", foo.x);
}
