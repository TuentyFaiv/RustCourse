use regex::Regex;

pub fn equation(rg: Regex, eq: &str, mut expresion: String) -> String {
  loop {
    // Apply operations
    let caps = rg.captures(expresion.as_str());

    if caps.is_none() {
      break;
    }

    let caps = caps.unwrap();

    let caps_expresion = caps.get(0).unwrap().as_str();
    let left_value: f64 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value: f64 = caps.get(2).unwrap().as_str().parse().unwrap();

    println!("=====CAPTURES=====");
    println!("{:?}, left: {:?} right: {:?}", caps, left_value, right_value);
    println!("=====CAPTURES=====");

    let result: f64;
    
    if eq == "division" {
      result = left_value / right_value;
    } else if eq == "multiply" {
      result = left_value * right_value;
    } else if eq == "add" {
      result = left_value + right_value;
    } else {
      result = left_value - right_value;
    }

    expresion = expresion.replace(caps_expresion, &result.to_string());
  }

  return expresion;
}

pub fn big_equation(rg: Regex, expresion: String) -> String {
  let caps = rg.captures(expresion.as_str());

  if caps.is_none() {
    return expresion;
  }
  let caps = caps.unwrap();

  let operations = caps.len();

  let mut sub_expresion = caps.get(0).unwrap().as_str().to_string();
  for op_index in 1..operations {
    
    let unformatted_exp = caps.get(op_index).unwrap().as_str().to_string();
    let mut operation = unformatted_exp.replace("(", "");
    operation = operation.replace(")", "");
    
    println!("=====OPERATION=====");
    println!("{:?}", operation);
    println!("=====OPERATION=====");

    operation = iterate_equiations(operation);

    sub_expresion = sub_expresion.replace(unformatted_exp.as_str(), &operation);
  
    println!("=====SUB EXPRESION=====");
    println!("{:?}", sub_expresion);
    println!("=====SUB EXPRESION=====");
  }
  return sub_expresion;
}

pub fn iterate_equiations(mut expresion: String) -> String {
  let re_div = Regex::new(format!(r"({REG_DIGIT})\s?/\s?({REG_DIGIT})").as_str()).unwrap();
  let re_mult = Regex::new(format!(r"({REG_DIGIT})\s?\*\s?({REG_DIGIT})").as_str()).unwrap();
  let re_add = Regex::new(format!(r"({REG_DIGIT})\s?\+\s?({REG_DIGIT})").as_str()).unwrap();
  let re_subs = Regex::new(format!(r"({REG_DIGIT})\s?\-\s?({REG_DIGIT})").as_str()).unwrap();

  expresion = equation(re_div, "division", expresion);
  expresion = equation(re_mult, "multiply", expresion);
  expresion = equation(re_add, "add", expresion);
  expresion = equation(re_subs, "subs", expresion);

  return expresion;
}

pub const REG_DIGIT: &str = r"\d+\.?\d{0,}";
pub const OPERATORS: &str = r"[/+\-*]?";
pub const ALL: &str = r"{0,}";