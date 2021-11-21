pub fn print() {
  let tuple = (1, 1.2, 5000);

  let (x, y, z) = tuple;

  let one = tuple.0;
  let one_point_two = tuple.1;

  println!("call tuple: {}, {}, {}, {}, {}", x, y, z, one, one_point_two);
}
