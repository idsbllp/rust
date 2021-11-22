fn if_else(num: i32) {
  if num < 5 {
    println!("condition is true");
  } else {
    println!("condition is false");
  }

  let result = if num < 5 { true } else { false };

  println!("control flow call: {}", result);
}

fn named_loop() {
  let mut outer = 0;

  'outer_loop: loop {
    print!("loop outer ====== {}:\t", outer);
    
    let mut inner = 10;
    loop {
      print!("loop inner, {} \t", inner);

      if inner < 10 {
        break;
      }

      if outer > 1 {
        break 'outer_loop;
      }

      inner -= 1;
    }

    println!();
    outer += 1;
  }

  println!("\nnamed_loop result outer: {}", outer);
}

fn return_loop() {
  let mut num = 0;

  let result = loop {
    if num == 2 {
      break "return value at break";
    }

    if num == 3 {
      break "never run at here";
    }

    num += 1;
  };

  println!("return_loop result: {}", result);
}

fn while_for() {
  let array = [1, 2, 3, 4, 5];
  let mut index = 0;
  let array_len = array.len();

  while index != array_len {
    print!("while print: {} \t", array[index]);
    index += 1;
  }

  println!();
  for value in array {
    print!("for in print: {} \t", value);
  }

  println!();
  for value in 1..3 {
    print!("for in and rev print: {} \t", value);
  }
}

pub fn print() {
  println!("================== control_flow start ================");
  if_else(3);
  named_loop();
  return_loop();
  while_for();
  println!("\n================== control_flow start ================");
}
