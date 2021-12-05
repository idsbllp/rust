mod if_let;

#[derive(Debug)]
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
  Vn,
}

impl IpAddr {
  fn print(&self) {
    println!("call print in IpAddr");
  }
}

fn get_ip_length(ip_addr: IpAddr) -> usize {
  match ip_addr {
    IpAddr::V4(_a, _b, _c, _d) => 4,
    IpAddr::V6(string) => string.len(),
    _ => 999,
  }
}

pub fn print() {
  let v4 = IpAddr::V4(127, 0, 0, 1);
  let v6 = IpAddr::V6(String::from("::1"));
  let vn = IpAddr::Vn;

  let v6_size = get_ip_length(v6);
  let vn_size = get_ip_length(vn);
  
  println!("\n\n================== enum start ================");
  println!("v6: {:?}", v6_size);
  println!("vn_size: {:?}", vn_size);
  v4.print();
  if_let::print();
  println!("================== enum end ================\n");
}
