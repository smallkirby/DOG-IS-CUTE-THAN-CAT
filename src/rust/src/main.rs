// 🐶 IS CUTE THAN 😼

use std::{panic, process::exit};

struct Cat {}
impl Cat {
  pub fn is_cute_than_dog(&self) -> bool {
    true
  }
}

fn main() {
  panic::set_hook(Box::new(|_| {
    println!("🐶 IS CUTE THAN 😼");
    exit(0);
  }));

  let cat = Cat {};
  match cat.is_cute_than_dog() {
    true => panic!("❓❓❓❓❓❓❓❓❓❓❓❓❓❓❓❓"),
    false => println!("💯"),
  }
}
