fn main() {
  let mut inp=get_line();
  let mut result: u32=0;
  let mut n: u32 = match inp.trim().parse() {
    Ok(num) => num,
    Err(_) => return,
  };

  loop {
    inp=get_line();
    let num: u32 = match inp.trim().parse() {
      Ok(num) => num,
      Err(_) => break,
    };

    let power: u32=num%10;
    let base: u32=num/10;
    result += base.pow(power);
    n-=1;
    if n == 0 {
      println!("{}", result);
      break;
    }
  }
}

fn get_line() -> String{
  // Get one line of input
  // Source: https://users.rust-lang.org/t/how-to-get-user-input/5176
  use std::io::{stdin,stdout,Write};
  let mut s=String::new();
  let _=stdout().flush();
  stdin().read_line(&mut s).expect("Did not enter a correct string");
  if let Some('\n')=s.chars().next_back() {
    s.pop();
  }
  if let Some('\r')=s.chars().next_back() {
    s.pop();
  }
  s
}
