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

/**
 * Using a hash map and vectors, create a text interface to allow a user
 * to add employeenames to a department in a company.
 * For example, "Add Sally to Engineering" or "Add Amir to Sales"
 * Then let the user retrieve a list of all people in a department
 * or all people in the company by department, sorted alphabetically.
 */
#[derive(Debug, Clone)]
struct Third {
  _department: HashMap<String, Vec<String>>,
}

impl Third {
  pub fn new() -> Third {
    Third { _department: HashMap::new() }
  }

  pub fn get_all_people(self, department: String) -> Result<Vec<String>, String> {
    let people = self._department.get(&department).ok_or("have no this department")?;
    let mut sorted_people = people.to_vec();
    sorted_people.sort();
    Ok(sorted_people)
  }

  pub fn add(&mut self, string: String) -> Result<(), String> {
    let string_vec: Vec<&str> = string.split(" ").collect();
    let &name = string_vec.get(1).ok_or("has no first value")?;
    let &department = string_vec.get(3).ok_or("has no third value")?;

    let new_people = self._department.entry(String::from(department)).or_insert(Vec::new());
    new_people.push(String::from(name));

    Ok(())
  }
}

pub fn print() {
  println!("\n================== exercises start ================");
  let let_q = vec![3, 2, 14, 53, 1, 14, 2, 43, 12, 45, 4];
  let first_a = first(let_q);

  let second_q = String::from("hello first apple world");
  let second_a = second(second_q);

  let mut third_q = Third::new();
  third_q.add(String::from("Add aaa to Engineering")).unwrap();
  third_q.add(String::from("Add eee to Engineering")).unwrap();
  third_q.add(String::from("Add ddd to Engineering")).unwrap();
  third_q.add(String::from("Add Amir to Sales")).unwrap();

  let third_a = third_q.clone().get_all_people(String::from("Engineering"));
  let third_b = third_q.clone().get_all_people(String::from("Sales"));

  println!("first anwser is: {:?}", first_a);
  println!("second anwser is: {:?}", second_a);
  println!("third a anwser is: {:?}", third_a);
  println!("third b anwser is: {:?}", third_b);
  println!("\n================== exercises end ================\n");

}
