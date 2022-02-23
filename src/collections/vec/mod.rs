pub fn print() {
  let mut v1: Vec<i32> = Vec::new();
  v1.push(11);
  v1.push(22);
  v1.push(33);

  let mut v2 = vec![1, 2, 3];
  v2.push(4);
  v2.push(5);
  v2.pop();

  // this 
  // let none_exist1 = &v1[100];
  let none_exist2 = v2.get(100);

  fn print_vector(vec: Vec<i32>) {
    for (index, el) in vec.iter().enumerate() {
      println!("iterating vector, index is: {}, item is: {}", index, el);
    }
  }

  fn update_vector(vec: &mut Vec<i32>) {
    for item in vec {
      *item += 30;
    }
  }

  #[derive(Debug)]
  enum SheetCell {
    Int(i32),
    Text(String),
  }

  let row = vec![
    SheetCell::Int(111),
    SheetCell::Text(String::from("eat eat??")),
    SheetCell::Text(String::from("no no")),
    SheetCell::Text(String::from("yes yes")),
  ];

  println!("\n================== vec start ================");
  println!("v1 is : {:?}", v1);
  println!("the index of 1 in v2 is: {}", &v2[1]);
  println!("the value of none_exist 2 is: {:?}", none_exist2);
  update_vector(&mut v2);
  print_vector(v2);
  println!("the vector with enum is: {:?}", row);
  println!("================== vec end ================\n");
}
