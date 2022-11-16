use sha2::{Digest, Sha512};
use std::iter::repeat_with;

static ALPHABET: &[u8] = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".as_bytes();

pub fn gen<'a, TString>(source: TString) -> String
where
  TString: AsRef<str>,
{
  let len = ALPHABET.len();
  let bytes = hash(source.as_ref().as_bytes());
  let mut res = String::with_capacity(512 >> 3);
  for byte in bytes {
    let index = byte as usize - (len * (byte as usize / len));
    let ch = ALPHABET[index] as char;
    res.push(ch)
  }
  res
}

pub fn gen_rnd() -> String {
  let source: String = repeat_with(|| fastrand::alphanumeric()).take(32).collect();
  gen(source)
}

pub fn hash(bytes: &[u8]) -> Vec<u8> {
  let mut hasher = Sha512::new();
  hasher.update(bytes);
  hasher.finalize()[..].into()
}
