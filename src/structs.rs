#[derive(Debug)]
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u32,
}

impl std::fmt::Display for User {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "(active: {}, username: {}, email: {}, sign_in_count: {})", self.active, self.username, self.email, self.sign_in_count)
  }
}

fn build_user(username: String) -> User {
  User {
    active: true,
    username: username,
    email: String::from("idsbllp@gmail.com"),
    sign_in_count: 1,
  }
}

#[derive(Debug)]
struct Point(i32, i32, i32);

struct AlwaysEqual;

pub fn print() {
  let mut me = build_user("idsbllp".to_string());

  let user = build_user(String::from("user"));
  let other_user = User {
    username: String::from("Tom and Jerry"),
    ..user
  };

  let point = Point(3, 2, 4);
  let x = point.0;

  let _subject = AlwaysEqual;
  let _object = AlwaysEqual;

  println!("\n\n================== struct start ================");
  println!("me: {:?}", me);
  me.email = String::from("liliping@gmail.com");
  println!("after modify me: {}", me);
  println!("other_user: {}", other_user);
  println!("point: {:?},   x: {1}", point, x);
  println!("================== struct end ================\n");
}
