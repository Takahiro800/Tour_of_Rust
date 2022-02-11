fn main() {
  let mut helloworld = String::from("hello");
  helloworld.push_str(" world");
  println!("{}", helloworld);
  helloworld = helloworld + "!";
  println!("{}", helloworld);
}
