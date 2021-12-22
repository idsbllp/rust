pub fn print() {
  // let str = String::from("string value");

  // let borrowed: &String = &str;
  // let copied: &String = borrowed;
  // let borrowed2: &&String = &borrowed;

  // println!("value   => s:{}, borrowed: {}, copied: {}, borrowed2: {}", str, borrowed, copied, borrowed2);
  // println!("pointers => borrowed: {:p}, copied: {:p} borrowed2: {:p}", borrowed, copied, borrowed2);

  let x = "string";
  let y = x;
  
  if x == y {
    println!("x is equal y !!!");
  } else {
    println!("x is not equal y ???");
  }

  println!("x is '{}', y is '{}'", x, y);

  let str = String::from(x);
  let len = calculate_length(&str);

  let mut mut_str = String::from(x);
  put_string(&mut mut_str);

  println!("x is '{}', its length is {}, new_str is '{}'", str, len, mut_str);
  mut_and_immut();
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn put_string(s: &mut String) {
  s.push_str(" world");
}

fn mut_and_immut() {
  let mut s = String::from("test mut_and_immut");

  let r1 = &s; // no problem
  let r2 = &s; // no problem
  println!("{} and {}", r1, r2);
  // variables r1 and r2 will not be used after this point

  let r3 = &mut s; // no problem
  println!("{}", r3);
}
