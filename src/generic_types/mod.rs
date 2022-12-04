// fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
fn get_largest<T>(list: &[T]) -> T
  where T: PartialOrd + Copy
{
  let mut max = list[0];

  for &item in list {
      if item > max {
          max = item;
      }
  }

  max
}

pub trait Summary {
  fn summarize_user(&self) -> String;

  fn summarize(&self) -> String {
    format!("summarize, {}", &self.summarize_user())
  }
}

struct News {
  pub name: String,
  pub headline: String,
}

impl Summary for News {
  fn summarize_user(&self) -> String {
    format!("{}", self.name)
  }

  fn summarize(&self) -> String {
    format!("News summary, name: {}, headline: {}", &self.name, &self.headline)
  }
}

struct Tweet {
  pub username: String,
  pub content: String,
}

impl Summary for Tweet {
  fn summarize_user(&self) -> String {
    format!("{}", self.username)
  }

  // fn summarize(&self) -> String {
  //   format!("Tweet, {} said: {}", &self.username, &self.content)
  // }
}

fn notify<T: Summary>(item: T) {
  println!("notify news: {}", item.summarize());
}

pub fn print() {
  let number_list = vec![1, 32, 10, 2, 8, 23, 42, 97];
  let result1 = get_largest(&number_list);

  let char_list = vec!['f', '3', 'h', '0', 'a', 'n', 'F'];
  let result2 = get_largest(&char_list);

  let news = News {
    name: String::from("Hurry"),
    headline: String::from("a moive"),
  };

  let twwet = Tweet {
    username: String::from("llp"),
    content: String::from("hurry is awesome"),
  };

  println!("================== generic types start ================");
  println!("max result of number list is: {}", result1);
  println!("max result of number list is: {}", result2);
  println!("{}", news.summarize());
  println!("{} said: {}\n", twwet.summarize(), twwet.content);
  notify(news);
  notify(twwet);
  println!("\n================== generic types end ================");
}
