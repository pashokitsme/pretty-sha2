#[cfg(test)]
pub mod tests {
  use std::iter::repeat_with;

  use pretty_hash::sha512;

  const LIMIT: usize = 15;

  #[test]
  fn simple() {
    for _ in 0..LIMIT {
      let random_str: String = repeat_with(|| fastrand::alphanumeric()).take(10).collect();
      assert_eq!(sha512::gen(&random_str), sha512::gen(&random_str));
    }
  }
}
