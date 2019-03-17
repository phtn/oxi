use std::io;

pub fn run(){
  let mut str = String::new();
  
  
  io::stdin().read_line(&mut str)
    .expect("failed.");

  let trm = str.trim();
  println!("subject: {}", str);
  println!("length: {}", trm.len());

  // for i in str.chars() {
  //   print!("{}", i)
  // }
}