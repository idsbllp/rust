pub fn print() {
  let array = [1, 22, 13, 44, 2, 5];
  let array2: [i32; 5] = [1, 22, 13, 44, 2];
  let array3: [i32; 5] = [3; 5];

  println!("array call: {}, {}, {}", array[1], array2[2], array3[2]);
}
