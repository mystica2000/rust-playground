use std::num::{IntErrorKind, ParseIntError};



#[derive(Debug)]
struct CustomIntError;

fn is_integer(str: &str) {
   str.parse::<i32>().unwrap_or(0);
   // str.parse::<i32>().unwrap(0);
   // str.parse::<i32>().expect("Not a integer");
}


fn is_integer_using_option(str: &str) -> Option<i32> {

  str.parse::<i32>().ok()
}

fn is_integer_using_result(str: &str) -> Result<i32, CustomIntError> {

  is_integer_using_option(str).ok_or(CustomIntError)
}

fn is_integer_to_propagate_option(str: &str) -> Option<i32>{
  Some(is_integer_using_option(str)?)
}

fn is_integer_to_propagate_result(str: &str) -> Result<i32, CustomIntError>{
  Ok(is_integer_using_result(str)?)
}

fn is_integer_result_to_option(str: &str) -> Option<i32> {
  is_integer_using_result(str).ok()
}

fn main() {
  println!("Hello, world!");

  match is_integer_using_option("test") {
    Some(num) => println!("Number is {}", num),
    None => println!("Some Error")
  }

  if let Some(val) = is_integer_using_option("12") {
    println!("Number is {}",val);
  }

  if let None = is_integer_using_option("test") {
    println!("Error - Abort");
  }

  println!("Result {:?}", is_integer_using_result("test").unwrap_err());

  is_integer("test");

  println!("Result {:?}", is_integer_using_result("test").map_err(|_| IntErrorKind::InvalidDigit).unwrap_err());



  println!("Result {:?}", is_integer_to_propagate_option("test"));


  println!("Result {:?}", is_integer_to_propagate_result("test").unwrap_err());
}
