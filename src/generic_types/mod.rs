fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut max = list[0];

  for &item in list {
      if item > max {
          max = item;
      }
  }

  max
}

pub fn print() {
  let number_list = vec![1, 32, 10, 2, 8, 23, 42, 97];
  let result1 = get_largest(&number_list);

  let char_list = vec!['f', '3', 'h', '0', 'a', 'n', 'F'];
  let result2 = get_largest(&char_list);

  println!("================== generic types start ================");
  println!("max result of number list is: {}", result1);
  println!("max result of number list is: {}", result2);
  println!("\n================== generic types end ================");
}
