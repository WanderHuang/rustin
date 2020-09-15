use super::parser::Ast;
use super::parser::AstTypeEnum;

pub fn from(ast: Ast) -> Result<String, String> {
  
  fn process(node: Ast) -> String {
    println!("Process => {:?} {:?}\n", node.ast_type, node);
    let res = match node.ast_type {
      AstTypeEnum::Program => {
        let mut res = String::from("");
        if let Some(vector) = node.body {
          for x in vector {
            res.push_str(&process(x));
            res.push_str("\n");
          }
        }
        res.pop();

        res
      },
      AstTypeEnum::ExpressionStatement => {
        let mut res = String::from("");
        if let Some(vector) = node.context {
            for x in vector {
              res.push_str(&process(x));
            }
        }
        res.push_str(";");
        res
      },
      AstTypeEnum::CallExpression => {
        let mut res = String::from("");
        if let Some(params) = node.params {
          res.push_str(&node.name.unwrap_or(String::from("Asynchronous")));
          res.push_str("(");
          for x in params {
            res.push_str(&process(x));
            res.push_str(", ");
          }
          res.pop();
          res.pop();
          res.push_str(")");
        }
        res
      },
      AstTypeEnum::Identifier => node.name.unwrap().to_owned(),
      AstTypeEnum::NumberLiteral => node.value.unwrap().to_owned(),
      AstTypeEnum::StringLiteral => {
        let mut res = String::from("");

        res.push_str("\"");
        res.push_str(&node.value.unwrap());
        res.push_str("\"");
        res
      }
      _ => String::from(""),
    };

    return res
  }
  
  Ok(
    process(ast)
  )
}