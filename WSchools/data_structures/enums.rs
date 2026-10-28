enum Direction {
  Up,
  Down,
  Left,
  Right,
}
// Enum variants can also hold data. This is useful when each variant needs to store extra information:

enum LoginStatus {
  Success(String),
  Error(String),
}

fn main() {
  let my_direction = Direction::Up;
  println!("We are going up!");

  match my_direction {
    Direction::Up => println!("Going up"),
    Direction::Down => println!("Going down"),
    Direction::Left => println!("Going left"),
    Direction::Right => println!("Going right"),
  }

  let result1 = LoginStatus::Success(String::from("Welcome, John!"));
  let result2 = LoginStatus::Error(String::from("Incorrect password"));

  match result1 {
    LoginStatus::Success(message) => println!("Success: {}",message),
    LoginStatus::Error(message) => println!("Error: {}",message),
  }

}