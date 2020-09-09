use crate::token::Token;
use crate::token::TokenEnum;
use std::error::Error;

#[derive(Debug, Clone)]
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
  ast_type: AstTypeEnum,
  value: Option<String>,
  name: Option<String>,
  params: Option<Vec<Ast>>,
  body: Option<Vec<Ast>>,
  parent: Option<Box<Ast>>,
  context: Option<Vec<Ast>>,
  callee: Option<Box<Ast>>,
  arguments: Option<Vec<Ast>>,
  expresssion: Option<Box<Ast>>,
}

impl Ast {
  fn new(ast_type: AstTypeEnum) -> Self {
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
  fn set_value(&mut self, val: String) -> Self {
    let mut next = self.clone();
    next.value = Some(val);
    next
  }

  fn set_name(&mut self, name: String) -> Self {
    let mut next = self.clone();
    next.name = Some(name);
    next
  }

  fn set_params(&mut self, arr: Vec<Ast>) -> Self {
    let mut next = self.clone();
    next.params = Some(arr);
    next
  }
  fn set_body(&mut self, bd: Vec<Ast>) -> Self {
    let mut next = self.clone();
    next.body = Some(bd);
    next
  }
  fn set_context(&mut self, context: Vec<Ast>) -> Self {
    let mut next = self.clone();
    next.context = Some(context);
    next
  }
  fn set_callee(&mut self, callee: Ast) -> Self {
    let mut next = self.clone();
    next.callee = Some(Box::new(callee));
    next
  }
  
  fn set_arguments(&mut self, args: Vec<Ast>) -> Self {
    let mut next = self.clone();
    next.arguments = Some(args);
    next

  }
  fn set_expression(&mut self, expresssion: Ast) -> Self {
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