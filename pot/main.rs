fn main() {
  let mut inp=get_line();
  let mut result: i32=0;
  let mut n: i32 = match inp.trim().parse() {
    Ok(num) => num,
    Err(_) => return,
  };

  loop {
    inp=get_line();
    let num: i32 = match inp.trim().parse() {
      Ok(num) => num,
      Err(_) => break,
    };

    let power=num%10;
    let base=num/10;
    result += pow(base, power);
    n-=1;
    if n == 0 {
      println!("{}", result);
      break;
    }
  }
}

fn pow(base: i32, power: i32) -> i32{
  let mut ans: i32=1;
  let mut count=power;
  loop {
    ans *= base;
    count-=1;
    if count==0 {
      break;
    }
  }

  ans
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
