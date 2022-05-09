// use std::ops::Add;

pub fn print() {
  let mut s = String::new();
  s.push_str("string");
  s = s + " added string";

  let s1 = "init string ";

  s.push_str(s1);
  s.push('c');

  let s2 = String::from(" s2");
  s = s + s1;
  s = s2 + &s;

  let index = "नमस्ते";
  let char1 = &index[0..6];

  println!("\n================== string start ================");
  println!("s is : {:?}, {:?}", s, s1);
  println!("slice string : {:?}\n", char1);
  for c in index.chars() {
    print!("{}  ", c);
  }
  println!("");
  for c in index.bytes() {
    print!("{}  ", c);
  }
  println!("\n================== string end ================\n");
}
