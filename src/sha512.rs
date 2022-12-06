use sha2::{Digest, Sha512};
use std::iter::repeat_with;

static ALPHABET: &[u8] = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".as_bytes();

pub fn gen<TString>(source: TString) -> String
where
  TString: AsRef<str>,
{
  let bytes = hash(source.as_ref().as_bytes());
  to_str(&bytes)
}

pub fn gen_rnd() -> String {
  let source: String = repeat_with(|| fastrand::alphanumeric()).take(32).collect();
  gen(source)
}

pub fn to_str(bytes: &[u8]) -> String {
  let len = ALPHABET.len();
  let mut res = String::with_capacity(512 >> 3);
  for &byte in bytes {
    let byte = byte as usize;
    let index = byte - (len * (byte / len));
    let ch = ALPHABET[index] as char;
    res.push(ch)
  }

  res
}

pub fn hash(bytes: &[u8]) -> Vec<u8> {
  let mut hasher = Sha512::new();
  hasher.update(bytes);
  hasher.finalize()[..].into()
}
