fn main() {
  // 文字をcharのベクトルとして集める
  let chars = "hi 🦀".chars().collect::<Vec<char>>();
  println!("{}", chars.len());

  // chars は 4 バイトなので、u32 に変換することができる
  println!("{}", chars[3] as u32);
  println!("{}", chars[0]);
}
