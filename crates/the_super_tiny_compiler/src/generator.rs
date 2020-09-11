use super::parser::Ast;
use super::parser::AstTypeEnum;

pub fn from(ast: Ast) -> Result<String, String> {
  
  fn process(node: Ast) -> String {
    println!("Process => {:?} {:?}", node.ast_type, node.value);
    let res = match node.ast_type {
      AstTypeEnum::Program => {
        let mut res = String::from("");
        for x in node.body.unwrap() {
          res.push_str(&process(x));
          res.push_str("\n");
        }

        res.pop();
        res
      },
      AstTypeEnum::ExpressionStatement => {
        let mut res = String::from("");
        res.push_str(&process(*node.expresssion.unwrap()));
        res.push_str(";");
        res
      },
      AstTypeEnum::CallExpression => {
        let mut res = String::from("");
        match node.clone().callee {
          Some(val) => {
            res.push_str(&process(*val));
            res.push_str("(");
            for x in node.arguments.unwrap() {
              res.push_str(&process(x));
            }
            res.push_str(")");
          }
          _ => return res
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