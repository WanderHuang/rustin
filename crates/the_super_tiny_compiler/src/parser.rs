use crate::token::Token;
use crate::token::TokenEnum;
use std::error::Error;
use std::cmp::PartialEq;

#[derive(Debug, Clone, PartialEq)]
pub enum AstTypeEnum {
  NumberLiteral,
  StringLiteral,
  CallExpression,
  ExpressionStatement,
  Program,
  Identifier,
  Null
}

#[derive(Debug, Clone)]
pub struct Ast {
  pub ast_type: AstTypeEnum,
  pub value: Option<String>,
  pub name: Option<String>,
  pub params: Option<Vec<Ast>>,
  pub body: Option<Vec<Ast>>,
  pub parent: Option<Box<Ast>>,
  pub context: Option<Vec<Ast>>,
  pub callee: Option<Box<Ast>>,
  pub arguments: Option<Vec<Ast>>,
  pub expresssion: Option<Box<Ast>>,
}

impl Ast {
  pub fn new(ast_type: AstTypeEnum) -> Self {
    Ast {
      ast_type,
      value: None,
      name: None,
      params: None,
      body: None,
      parent: None,
      context: None,
      callee: None,
      arguments: None,
      expresssion: None,
    }
  }
  pub fn set_value(&mut self, val: String) -> Self {
    let mut next = self.clone();
    next.value = Some(val);
    next
  }

  pub fn set_name(&mut self, name: String) -> Self {
    let mut next = self.clone();
    next.name = Some(name);
    next
  }

  pub fn set_params(&mut self, arr: Vec<Ast>) -> Self {
    let mut next = self.clone();
    next.params = Some(arr);
    next
  }
  pub fn set_body(&mut self, bd: Vec<Ast>) -> Self {
    let mut next = self.clone();
    next.body = Some(bd);
    next
  }
  pub fn set_context(&mut self, context: Vec<Ast>) -> Self {
    let mut next = self.clone();
    next.context = Some(context);
    next
  }
  // pub fn set_callee(&mut self, callee: Ast) -> Self {
  //   let mut next = self.clone();
  //   next.callee = Some(Box::new(callee));
  //   next
  // }
  
  // pub fn set_arguments(&mut self, args: Vec<Ast>) -> Self {
  //   let mut next = self.clone();
  //   next.arguments = Some(args);
  //   next

  // }

  pub fn set_expression(&mut self, expresssion: Ast) -> Self {
    let mut next = self.clone();
    next.expresssion = Some(Box::new(expresssion));
    next

  }


}

pub trait AstReader {
  fn transform(tokens: &mut Vec<Token>) -> Ast;
}

impl AstReader for Ast {
  fn transform(tokens: &mut Vec<Token>) -> Ast {
    
    fn walk(tokens: &mut Vec<Token>) -> Result<Ast, Box<dyn Error>>{
      
      let op_token = get_token(tokens);
      match op_token {
        Some(token) => {
          let mut token = token;
          match token.token_type {
            TokenEnum::Number => {
              println!("Number {:?}", token);
              return Ok(
                Ast::new(AstTypeEnum::NumberLiteral).set_value(token.value)
              );
            },
            TokenEnum::Quote => {
              return Ok(
                Ast::new(AstTypeEnum::StringLiteral).set_value(token.value)
              );
            },
            TokenEnum::Name => {
              let name = token.value;
              token = get_token(tokens).unwrap();
              if token.value == "(" {
                let mut expresssion = Ast::new(AstTypeEnum::CallExpression).set_name(name).set_params(Vec::new());
      
                let mut params = expresssion.params.unwrap();
                while !TokenEnum::Paren.eq(&token.token_type) || (TokenEnum::Paren.eq(&token.token_type) && token.value != ")") {
                  let last = tokens.last().unwrap();
                  if last.value == ")" {
                    get_token(tokens);
                    break;
                  }
                  let res = walk(tokens);
                  if res.is_ok() {
                    params.push(res.unwrap());
                  }
                }
    
                println!("token {:?}", token);
    
                expresssion.params = Some(params);
                return Ok(
                  expresssion
                );
              } else {
                panic!("Cannot parse this token, {:?}", token);
              }
            },
            _ => {
              println!("Nothing to do");
            },
          }
          panic!("Error, this token is illegal, {:?}", token);
        },
        None => {
          panic!("Noting to do")
        }
      }
      

    };

    fn get_token(tokens: &mut Vec<Token>) -> Option<Token> {
      tokens.pop()
    }

    let mut ast = Ast::new(AstTypeEnum::Program).set_body(Vec::new());

    let mut bd = ast.body.unwrap();
    while tokens.len() > 0 {
      let res = walk(tokens);
      if res.is_ok() {
        bd.push(res.unwrap());
      }
    }

    ast.body = Some(bd);

    return ast;
  }
}

#[cfg(test)]
mod tests {

  use crate::parser::Ast;
  use crate::parser::AstReader;
  use crate::token::Token;
  use crate::token::TokenEnum;
  
  #[test]
  fn test_parse() {
    let mut tokens = Vec::new();
    tokens.push(Token::new(TokenEnum::Name, "add"));
    tokens.push(Token::new(TokenEnum::Paren, "("));
    tokens.push(Token::new(TokenEnum::Number, "123"));
    tokens.push(Token::new(TokenEnum::Number, "678"));
    tokens.push(Token::new(TokenEnum::Paren, ")"));

    tokens.reverse();

    let ast = Ast::transform(&mut tokens);

    println!("Ast {:?}", ast);
  }
}