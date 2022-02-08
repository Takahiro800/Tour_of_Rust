fn do_something_that_fail(i: i32) -> Result<f32, String> {
  if i == 42 {
    Ok(13.0)
  } else {
    Err(String::from("正しい値ではありません"))
  }
}

fn main() {
  let result = do_something_that_fail(12);

  match result {
    Ok(v) => println!("発見 {}", v),
    Err(e) => println!("Error: {}", e),
  }

  let result = do_something_that_fail(42);
  match result {
    Ok(v) => println!("発見 {}", v),
    Err(e) => println!("Error: {}", e),
  }
}
