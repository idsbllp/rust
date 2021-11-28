
pub fn print() {
  let str = String::from("string value");

  let borrowed: &String = &str;
  let copied: &String = borrowed;
  let borrowed2: &&String = &borrowed;



  println!("\n================== owner_ship start ================");
  println!("value   => s:{}, borrowed: {}, copied: {}, borrowed2: {}", str, borrowed, copied, borrowed2);
  println!("pointers => borrowed: {:p}, copied: {:p} borrowed2: {:p}", borrowed, copied, borrowed2);
  println!("================== owner_ship end ================\n");
}
