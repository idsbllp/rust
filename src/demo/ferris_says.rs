use std::io::{stdout, BufWriter};
use ferris_says::say;

pub fn print() {
  let stdout = stdout();
  let message = String::from("Hello Rust, are you ok?");
  let width = message.chars().count();

  let mut writer = BufWriter::new(stdout.lock());
  say(message.as_bytes(), width, &mut writer).unwrap();
}
