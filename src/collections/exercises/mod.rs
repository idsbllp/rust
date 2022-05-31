use std::collections::HashMap;

/**
 * Given a list of integers, use a vector and 
 * return the median (when sorted, the value in the middle position) 
 * and mode (the value that occurs most often; a hash map will be helpful here) of the list.
 */
fn first(list: Vec<i32>) -> (i32, i32) {
  let mut sorted_list = list.clone();
  sorted_list.sort();
  let count = sorted_list.len();
  let median = sorted_list[count / 2];

  let mut map = HashMap::new();

  for value in list {
    let v = map.entry(value).or_insert(0);
    *v +=1;
  }

  let mut mode1 = 0;
  let mut mode_count = 0;

  for (num, val) in map {
    if mode_count < val {
      mode_count = val;
      mode1 = num;
    }
  }

  (median, mode1)
}

pub fn print() {
  let let_q = vec![3, 2, 14, 53, 1, 14, 2, 43, 12, 45, 4];
  let first_a = first(let_q);

  println!("\n================== exercises start ================");
  println!("first anwser should be 12: {:?}", first_a);
  println!("\n================== exercises end ================\n");

}
