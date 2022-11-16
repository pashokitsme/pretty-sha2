A wrapper for sha2 library to get hash string with \[0-9\] & \[a-z\] & \[A-Z\] alphabet

Code example

```rust
fn main() {
  let some_input = "some string";
  let hashed = pretty_sha2::sha512::gen(some_input);
  let random_hash = pretty_sha2::sha512::gen_rnd();
  println!("{}", hashed); // KMW1haL81kP6KR4vwpQoxRngltgmiEoXIYATz6klwmvmIx62mulnQrHscoAy29k7
  println!("{}", random_hash)
}

```
