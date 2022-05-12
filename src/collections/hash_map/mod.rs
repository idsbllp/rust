use std::collections::HashMap;

fn word_count() -> HashMap<String, i32> {
  let text = "hello, world";

  let mut map = HashMap::new();

  for chars in text.split("") {
    let count = map.entry(String::from(chars)).or_insert(0);
    *count += 1;
  }

  map
}

pub fn print() {
  let mut scores = HashMap::new();

  scores.insert("A", 40);
  scores.insert("B", 41);

  let classes = vec!["C", "D", "E", "F"];
  let score = vec![50, 60];

  let mut scores1: HashMap<_, _> = classes.clone().into_iter().zip(score.clone().into_iter()).collect();
  let scores2: HashMap<&str, i32> = classes.into_iter().zip(score.into_iter()).collect();
  
  scores1.entry("C").or_insert(70);
  scores1.entry("E").or_insert(70);

  let c_score = scores1.get("C");

  println!("\n================== hash map start ================");
  println!("{:?}", scores);
  println!("{:?}", scores1);
  println!("{:?}", scores2);
  println!("{:?}", c_score);

  for (key, value) in scores {
    print!("{}: {} and ", key, value);
  }

  println!("word count: {:?}", word_count());
  println!("\n================== hash map end ================\n");
}
