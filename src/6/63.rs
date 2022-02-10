fn main() {
  let a: &'static str = r#"
    <div class="advice">
      生文字列は様々な場面で役に立ちます。
    </div>
  "#;
  println!("{}", a);
}
