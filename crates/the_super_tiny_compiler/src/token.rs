use regex::Regex;
use std::cmp::PartialEq;

pub trait TokenReader {
  fn read(input: &str) -> Vec<Token>;
  fn print(token: Token);
}

#[derive(Debug, PartialEq)]
pub enum TokenEnum {
  Paren,
  Number,
  Quote,
  Name,
  // This
}


#[derive(Debug)]
pub struct Token {
  pub token_type: TokenEnum,
  pub value: String,
}

impl Token { 
  pub fn new(token_type: TokenEnum, value: &str) -> Self {
    Token {
      token_type,
      value: String::from(value)
    }
  }
}


impl TokenReader for Token {
  fn read(input: &str) -> Vec<Token> {

    let mut current = 0;
    let mut tokens = Vec::new();

    let space_test = Regex::new(r"\s").unwrap();
    let number_test = Regex::new(r"\d").unwrap();
    let char_test = Regex::new(r"([a-z])").unwrap();

    println!("length = {:?}", input.chars().nth(9));

    while let Some(c) = input.chars().nth(current) {
      if c == '(' || c == ')' {
        tokens.push(Token::new(TokenEnum::Paren, &c.to_string()));
        current += 1;
        continue;
      }

      if space_test.is_match(&c.to_string()) {
        current += 1;
        continue;
      }

      if let Some(x) = input.chars().nth(current) {
        if number_test.is_match(&x.to_string()) {
          let mut val = String::from("");
          while let Some(x) = input.chars().nth(current) {
            if number_test.is_match(&x.to_string()) {
              val += &x.to_string();
              current += 1;
            } else {
              break;
            }
          }

          tokens.push(Token::new(TokenEnum::Number, &val));

          continue;
        }
      }

      

      if let Some(x) = input.chars().nth(current) {
        if char_test.is_match(&x.to_string()) {
          let mut val = String::from("");
          while let Some(x) = input.chars().nth(current) {
            if char_test.is_match(&x.to_string()) {
              val += &x.to_string();
              current += 1;
            } else {
              break;
            }
          }
  
          tokens.push(Token::new(TokenEnum::Name, &val));
  
          continue;
        }
      }

      if c == '"' {
        let mut val = String::from("");
        current += 1;
        while let Some(q) = input.chars().nth(current) {
          if q != '"' {
            val += &q.to_string();
            current += 1;
          } else {
            current += 1;
            break;
          }
        }
        tokens.push(Token::new(TokenEnum::Quote, &val));
        continue;
      }
    }

    return tokens;
  }

  fn print(token: Token) {
    println!("Token {{token_type: {:?}, value: {}}}", token.token_type, token.value);
  }
}
