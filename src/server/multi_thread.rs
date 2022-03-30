use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use thread_pool::ThreadPool;

// mod thread_pool;

pub fn start() {
  let listener = TcpListener::bind("127.0.0.1:5757").unwrap();
  let pool = ThreadPool::new(4);

  println!("Server running at http://127.0.0.1:5757");

  for stream in listener.incoming() {
    let new_stream = stream.unwrap();

    pool.execute(|| {
      handle_stream(new_stream);
    });
  }
}

fn handle_stream(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";
  let path_prefix = "src/server/";
  let sleep = b"GET /sleep HTTP/1.1\r\n";

  let (status, file) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK\r\n", format!("{}index.html", path_prefix))
  } else if buffer.starts_with(sleep) {
    thread::sleep(Duration::from_secs(5));
    ("HTTP/1.1 200 OK\r\n", format!("{}index.html", path_prefix))
  }
  else {
    ("HTTP/1.1 404 NOT FOUND\r\n", format!("{}404.html", path_prefix))
  };

  let content = fs::read_to_string(file).unwrap();
  let response = format!("{}Content-length: {}\r\n\r\n {}", status, content.len(), content);

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();

  
  // println!("Request is : {}", String::from_utf8_lossy(&buffer));
}
