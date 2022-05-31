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

/**
 * Convert strings to pig latin. The first consonant of each word is moved to 
 * the end of the word and “ay” is added, so “first” becomes “irst-fay.” 
 * Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
 * Keep in mind the details about UTF-8 encoding!
 */
fn second(string: String) -> String {
  let string_arr: Vec<&str> = string.split(' ').collect();
  
  fn is_start_with_vowel(word: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let _is_start_with_vowel = vowels.iter().find(|&&x| word.starts_with(x));

    if matches!(_is_start_with_vowel, None) {
      return false;
    }

    return true;
  }

  let mut res = String::from("");
  for word in string_arr {
    if is_start_with_vowel(word) {
      res = format!("{} {}-hay", res, word);
    } else {
      res = format!("{} {}-{}ay", res, &word[1..], &word[..1]);
    }
  }

  return res;
}

pub fn print() {
  println!("\n================== exercises start ================");
  let let_q = vec![3, 2, 14, 53, 1, 14, 2, 43, 12, 45, 4];
  let first_a = first(let_q);

  let second_q = String::from("hello first apple world");
  let second_a = second(second_q);

  println!("first anwser should be 12: {:?}", first_a);
  println!("second anwser should be 12: {:?}", second_a);
  println!("\n================== exercises end ================\n");

}
