fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
  if i == 42 {
    Ok(13.0)
  } else {
    Err(String::from("正しい値ではありません"))
  }
}

fn main() -> Result<(), String> {
  let v = do_something_that_might_fail(421)?;
  println!("発見 {}", v);
  Ok(())
}
