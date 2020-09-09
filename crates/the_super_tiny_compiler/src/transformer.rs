use super::parser::Ast;
use super::parser::AstTypeEnum;


pub fn generate(ast: Ast) -> Ast {
  let nextAst = Ast::new(AstTypeEnum::Program).set_body(Vec::new());
  traverseNode(ast, Ast::new(AstTypeEnum::Null), |node, parent| {
    match node.ast_type {
      AstTypeEnum::CallExpression => {
        let mut expression = Ast::new(node.ast_type)
        .set_callee(Ast::new(AstTypeEnum::Identifier).set_name(node.name))
        .set_arguments(Vec::new());

        node.set_context(expression.arguments);

        if !AstTypeEnum::CallExpression.eq(parent.ast_type) {
          expression = Ast::new(AstTypeEnum::ExpressionStatement).set_expression(expression);
        }

        parent.context.push(expression);
      },
      _ => {
        if parent.context == None {
          parent.context = Vec::new();
        }
        parent.context.push(Box::new(Ast::new(node.ast_type).set_value(node.value)));
      },

    }
  });

  return nextAst;
}

fn traverseNode(node: Ast, parent: Ast, visitor: F) where F: Fn(Ast, Ast) {
  visitor(node, parent);

  match node.ast_type {
    AstTypeEnum::Program => traverseArray(node.body, node),
    AstTypeEnum::CallExpression => traverseArray(node.params, node),
    _ => println!("traverse on node type = {}", node.ast_type),
  }
}

fn traverseArray(array: Vec<Ast>, parent: Ast)  {
  for x in array {
    traverseNode(x, parent);
  }
}