fn plus_one(num: Option<i32>) -> Option<i32> {
  match num {
    Some(i) => Some(i + 1),
    None => None,
  }
}

pub fn print() {
  let some_max = None;
  
  let value1: Option<u16> = match some_max {
    Some(max) => Some(max),
    None => None,
  };

  println!("\nvalue1 with match is : {:?}", value1);

  if let Some(value2) = some_max {
    println!("value2 with if_let is some: {}", value2);
  } else {
    println!("value2 must be None!");
  }
  println!("plus_one with 12: {:?}", plus_one(Some(12)));
  println!("plus_one with None: {:?}", plus_one(None));
}
