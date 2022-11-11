use regex::Regex;
use crate::utils::{equations::{ big_equation, iterate_equiations, REG_DIGIT, OPERATORS, ALL}};

pub fn start() {  
  // Regex
  const DIGITS: &str = REG_DIGIT;
  let reg_operation = format!(r"(?:{OPERATORS}{DIGITS}{OPERATORS}){ALL}");
  let re_paren = Regex::new(format!(r"(\({reg_operation}\)){reg_operation}(\({reg_operation}\))").as_str()).unwrap();
  
  // Get user data
  println!("Please enter your expresion");

  let mut expresion = String::new();

  std::io::stdin().read_line(&mut expresion).unwrap();

  // Apply equations
  expresion = big_equation(re_paren, expresion);
  expresion = iterate_equiations(expresion);

  let result: f64 = expresion.trim().parse().unwrap_or(0.0);

  println!("=====RESULT=====");
  println!("{}", result);
  println!("=====RESULT=====");
}