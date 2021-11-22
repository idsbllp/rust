mod number;
mod string;
mod tuple;
mod array;

pub fn print() {
  println!("================== types start ================");
  number::print();
  string::print();
  tuple::print();
  array::print();
  println!("================== types end ================");
}
