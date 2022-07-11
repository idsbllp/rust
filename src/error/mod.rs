use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn print1() {
  let file_path = "src/error/hello1.test.md";
  let file = File::open(file_path);

  let mut file = match file {
    Ok(f) => f,
    Err(err) => match err.kind() {
      ErrorKind::NotFound => match File::create(file_path) {
        Ok(f) => f,
        Err(err) => panic!("create file failed: {:?}", err),
      },
      other_error => panic!("open file failed: {:?}", other_error),
    }
  };

  let mut buf = String::new();
  let s = match file.read_to_string(&mut buf) {
    Ok(_) => buf,
    Err(_) => panic!("error ")
  };
  println!("s {}", s);
}

fn print2() {
  let file_path = "src/error/hello1.test.md";

  File::open(file_path).unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create(file_path).unwrap_or_else(|error| {
        panic!("create file failed: {:?}", error);
      })
    } else {
      panic!("open file failed: {:?}", error);
    }
  });
}

fn print3() {
  let file_path = "src/error/hello1.test.md";

  File::open(file_path).unwrap();
  File::open(file_path).expect("open file failed");
}

pub fn print() {
  println!("================== error handle start ================");
  print1();
  print2();
  print3();
  println!("\n================== error handle start ================");
}
